use super::*;

// ─── Cost budget regression guards (issue #227) ─────────────────────────────
//
// Thresholds are documented in docs/GAS_AND_FEES.md. They sit far above the
// costs currently measured in the native-Rust test environment, so only a
// large cost regression trips them. The test budget resets before every
// top-level contract invocation, so each test below meters exactly the one
// call named in its name.

const CPU_INSTRUCTION_COST_LIMIT: u64 = 10_000_000;
const MEMORY_BYTES_COST_LIMIT: u64 = 1_000_000;

fn assert_budget_under_thresholds(env: &Env) {
    let budget = env.cost_estimate().budget();
    let cpu = budget.cpu_instruction_cost();
    let memory = budget.memory_bytes_cost();
    assert!(
        cpu < CPU_INSTRUCTION_COST_LIMIT,
        "cpu instruction cost {cpu} exceeded limit {CPU_INSTRUCTION_COST_LIMIT}"
    );
    assert!(
        memory < MEMORY_BYTES_COST_LIMIT,
        "memory bytes cost {memory} exceeded limit {MEMORY_BYTES_COST_LIMIT}"
    );
}

#[test]
fn issue_ticket_stays_under_cost_thresholds() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);

    let buyer = Address::generate(&env);
    client.issue_ticket(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "unassigned"),
        &1_000i128,
    );
    assert_budget_under_thresholds(&env);
}

#[test]
fn verify_ticket_stays_under_cost_thresholds() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let buyer = Address::generate(&env);
    let ticket_id = client.issue_ticket(
        &organizer,
        &1,
        &buyer,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "unassigned"),
        &1_000i128,
    );

    client.verify_ticket(&ticket_id);
    assert_budget_under_thresholds(&env);
}

#[test]
fn buy_resale_stays_under_cost_thresholds() {
    let (env, client, token, token_asset, _admin, organizer) = setup();
    make_event(&env, &client, &organizer, 1);
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let ticket_id = client.issue_ticket(
        &organizer,
        &1,
        &seller,
        &String::from_str(&env, "GA"),
        &String::from_str(&env, "unassigned"),
        &1_000i128,
    );
    client.list_for_resale(&seller, &ticket_id, &1_100i128);
    token_asset.mint(&buyer, &10_000i128);

    client.buy_resale(&buyer, &ticket_id);
    assert_eq!(token.balance(&organizer), 55);
    assert_budget_under_thresholds(&env);
}
