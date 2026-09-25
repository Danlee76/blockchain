#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env, String,
};

pub struct TestContext<'a> {
    pub env: Env,
    pub client: TicketingContractClient<'a>,
    pub token: TokenClient<'a>,
    pub token_asset: StellarAssetClient<'a>,
    pub admin: Address,
    pub organizer: Address,
}

/// Standard contract and token setup for ticketing tests.
pub fn setup<'a>() -> (
    Env,
    TicketingContractClient<'a>,
    TokenClient<'a>,
    StellarAssetClient<'a>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let organizer = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = TokenClient::new(&env, &token_contract.address());
    let token_asset = StellarAssetClient::new(&env, &token_contract.address());

    let contract_id = env.register(TicketingContract, ());
    let client = TicketingContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_contract.address());

    (env, client, token, token_asset, admin, organizer)
}

/// Creates a default concert event with standard resale cap and royalty.
pub fn make_event(env: &Env, client: &TicketingContractClient, organizer: &Address, event_id: u64) {
    client.create_event(
        organizer,
        &event_id,
        &String::from_str(env, "Radiohead Live"),
        &String::from_str(env, "concert"),
        &12_000u32, // max 120% of face value on resale
        &500u32,    // 5% organizer royalty
        &10_000u64,
        &100u64,
        &200u64,
    );
}

/// Creates an event with custom parameters.
pub fn make_custom_event(
    env: &Env,
    client: &TicketingContractClient,
    organizer: &Address,
    event_id: u64,
    name: &str,
    category: &str,
    max_resale_multiplier_bps: u32,
    royalty_bps: u32,
    starts_at: u64,
) {
    client.create_event(
        organizer,
        &event_id,
        &String::from_str(env, name),
        &String::from_str(env, category),
        &max_resale_multiplier_bps,
        &royalty_bps,
        &starts_at,
        &100u64,
        &200u64,
    );
}

/// Helper to mint tokens to an address.
pub fn mint_tokens(token_asset: &StellarAssetClient, recipient: &Address, amount: i128) {
    token_asset.mint(recipient, &amount);
}

/// Helper to issue a standard GA ticket.
pub fn issue_sample_ticket(
    env: &Env,
    client: &TicketingContractClient,
    organizer: &Address,
    event_id: u64,
    buyer: &Address,
    price: i128,
) -> u64 {
    client.issue_ticket(
        organizer,
        &event_id,
        buyer,
        &String::from_str(env, "GA"),
        &String::from_str(env, "unassigned"),
        &price,
    )
}
