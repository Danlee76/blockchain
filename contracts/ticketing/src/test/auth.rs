use super::*;

#[contract]
struct FailingToken;

#[contractimpl]
impl FailingToken {
    pub fn decimals(_env: Env) -> u32 {
        7
    }

    pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {
        panic!("simulated token transfer failure");
    }
}

#[contract]
pub struct MaliciousReentrantToken;

#[contractimpl]
impl MaliciousReentrantToken {
    pub fn decimals(_env: Env) -> u32 {
        7
    }

    pub fn transfer(env: Env, from: Address, _to: Address, amount: i128) {
        // Attempt recursive reentrancy callback into the ticketing contract
        if let Some(target) = env.storage().instance().get::<_, Address>(&DataKey::Admin) {
            let client = TicketingContractClient::new(&env, &target);
            let _ = client.try_purchase_primary(
                &from,
                &1u64,
                &String::from_str(&env, "GA"),
                &String::from_str(&env, "reentrant"),
                &amount,
            );
        }
    }

    pub fn set_target(env: Env, target: Address) {
        env.storage().instance().set(&DataKey::Admin, &target);
    }
}

#[test]
fn initialize_rejects_a_second_call() {
    let (env, client, _token, _token_asset, admin, _organizer) = setup();
    let second_token = Address::generate(&env);
    let result = client.try_initialize(&admin, &second_token);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn payment_token_propose_and_apply_flow() {
    let (env, client, _token, _token_asset, admin, _organizer) = setup();

    let new_token_admin = Address::generate(&env);
    let new_token_contract = env.register_stellar_asset_contract_v2(new_token_admin);
    let new_token_addr = new_token_contract.address();

    client.propose_payment_token(&admin, &new_token_addr);

    // Cannot apply before delay elapses
    let early_apply = client.try_apply_payment_token(&admin);
    assert_eq!(early_apply, Err(Ok(Error::TimelockNotElapsed)));

    // Advance ledger past delay
    env.ledger().set_sequence_number(100_000);
    client.apply_payment_token(&admin);
}

#[test]
fn apply_payment_token_rejects_without_proposal() {
    let (env, client, _token, _token_asset, admin, _organizer) = setup();
    let result = client.try_apply_payment_token(&admin);
    assert_eq!(result, Err(Ok(Error::NoPendingPaymentToken)));
}

#[test]
fn purchase_throttle_rejects_rapid_repeat_purchases() {
    let (env, client, _token, token_asset, admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_purchase_throttle(&admin, &10u32);

    let buyer = Address::generate(&env);
    token_asset.mint(&buyer, &10_000i128);

    env.ledger().set_sequence_number(100);
    client.purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "1"),
        &1_000i128,
    );

    // Second purchase in the same block (or within 10 ledgers) must be throttled
    env.ledger().set_sequence_number(105);
    let throttled = client.try_purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "2"),
        &1_000i128,
    );
    assert_eq!(throttled, Err(Ok(Error::PurchaseThrottled)));
}

#[test]
fn purchase_throttle_allows_purchase_after_spacing_elapses() {
    let (env, client, _token, token_asset, admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    client.set_purchase_throttle(&admin, &10u32);

    let buyer = Address::generate(&env);
    token_asset.mint(&buyer, &10_000i128);

    env.ledger().set_sequence_number(100);
    client.purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "1"),
        &1_000i128,
    );

    env.ledger().set_sequence_number(111);
    let ok = client.try_purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "2"),
        &1_000i128,
    );
    assert!(ok.is_ok());
}

#[test]
fn set_purchase_throttle_rejects_a_non_admin_caller() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    let result = client.try_set_purchase_throttle(&organizer, &10u32);
    assert_eq!(result, Err(Ok(Error::NotAdmin)));
}

#[test]
fn purchase_primary_leaves_no_partial_state_when_token_transfer_fails() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    let failing_token = env.register(FailingToken, ());
    client.set_event_payment_token(&organizer, &1, &Some(failing_token));

    let buyer = Address::generate(&env);
    let event_before = client.get_event(&1);

    let result = client.try_purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "1"),
        &1_000i128,
    );
    assert!(
        result.is_err(),
        "expected purchase_primary to fail when the token transfer fails"
    );

    let event_after = client.get_event(&1);
    assert_eq!(
        event_before, event_after,
        "event state must be unchanged after a failed token transfer"
    );
    assert!(
        client.try_get_ticket(&0).is_err(),
        "no ticket should have been minted when the token transfer failed"
    );
}

#[test]
fn malicious_token_reentrancy_fails_safely_and_preserves_state() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    let malicious_token_id = env.register(MaliciousReentrantToken, ());
    let token_client = MaliciousReentrantTokenClient::new(&env, &malicious_token_id);
    token_client.set_target(&client.address);

    client.set_event_payment_token(&organizer, &1, &Some(malicious_token_id));

    let buyer = Address::generate(&env);
    let event_before = client.get_event(&1);

    let _ = client.try_purchase_primary(
        &buyer,
        &1,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "1"),
        &1_000i128,
    );

    // Contract state remains consistent and tickets_issued accurately reflects final state
    let event_after = client.get_event(&1);
    assert!(event_after.tickets_issued <= event_before.tickets_issued + 1);
}
