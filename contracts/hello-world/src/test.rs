#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, String, BytesN,
};

mod contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/opentender_contract.wasm"
    );
}

use contract::{OpenTenderContractClient, Tender, Bid};

fn create_test_contract(env: &Env) -> (OpenTenderContractClient, Address) {
    let contract_id = env.register_contract(None, contract::WASM);
    let client = OpenTenderContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    
    client.initialize(&admin);
    
    (client, admin)
}

fn advance_ledger_time(env: &Env, seconds: u64) {
    let current = env.ledger().timestamp();
    env.ledger().set(LedgerInfo {
        timestamp: current + seconds,
        protocol_version: 20,
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 16,
        max_entry_ttl: 6312000,
    });
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, contract::WASM);
    let client = OpenTenderContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    
    client.initialize(&admin);
    
    // Verify initialization by checking tender count
    let count = client.get_tender_count();
    assert_eq!(count, 0);
}

#[test]
#[should_panic(expected = "Admin not set")]
fn test_initialize_only_once() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    
    // Try to initialize again - should panic
    let another_admin = Address::generate(&env);
    client.initialize(&another_admin);
}

#[test]
fn test_create_tender() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 86400; // 1 day
    let reveal_deadline = deadline + 86400; // 2 days
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Road Construction Project"),
        &String::from_str(&env, "QmTestHash123"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    assert_eq!(tender_id, 1);
    
    // Verify tender was created
    let tender = client.get_tender(&tender_id);
    assert_eq!(tender.id, 1);
    assert_eq!(tender.creator, creator);
    assert_eq!(tender.deadline, deadline);
    assert_eq!(tender.reveal_deadline, reveal_deadline);
    assert_eq!(tender.min_bid, 100000i128);
    assert_eq!(tender.is_closed, false);
}

#[test]
fn test_create_multiple_tenders() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 86400;
    let reveal_deadline = deadline + 86400;
    
    // Create first tender
    let tender_id1 = client.create_tender(
        &creator,
        &String::from_str(&env, "Project 1"),
        &String::from_str(&env, "QmHash1"),
        &deadline,
        &reveal_deadline,
        &50000i128,
    );
    
    // Create second tender
    let tender_id2 = client.create_tender(
        &creator,
        &String::from_str(&env, "Project 2"),
        &String::from_str(&env, "QmHash2"),
        &deadline,
        &reveal_deadline,
        &75000i128,
    );
    
    assert_eq!(tender_id1, 1);
    assert_eq!(tender_id2, 2);
    
    let count = client.get_tender_count();
    assert_eq!(count, 2);
}

#[test]
#[should_panic(expected = "Invalid deadlines")]
fn test_create_tender_invalid_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time - 1000; // Past deadline
    let reveal_deadline = deadline + 86400;
    
    // Should panic due to invalid deadline
    client.create_tender(
        &creator,
        &String::from_str(&env, "Invalid Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
}

#[test]
#[should_panic(expected = "Invalid deadlines")]
fn test_create_tender_reveal_before_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 86400;
    let reveal_deadline = deadline - 1000; // Before bidding deadline
    
    // Should panic due to reveal deadline before bidding deadline
    client.create_tender(
        &creator,
        &String::from_str(&env, "Invalid Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
}

#[test]
fn test_commit_bid() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 86400;
    let reveal_deadline = deadline + 86400;
    
    // Create tender
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Create bid hash (simplified - in production use proper SHA-256)
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    
    // Commit bid
    client.commit_bid(&tender_id, &bidder, &bid_hash);
    
    // Verify bid was committed
    let bid = client.get_bid(&tender_id, &bidder);
    assert_eq!(bid.bidder, bidder);
    assert_eq!(bid.tender_id, tender_id);
    assert_eq!(bid.commit_hash, bid_hash);
    assert_eq!(bid.is_valid, false); // Not yet revealed
}

#[test]
fn test_commit_multiple_bids() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 86400;
    let reveal_deadline = deadline + 86400;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Create multiple bidders
    let bidder1 = Address::generate(&env);
    let bidder2 = Address::generate(&env);
    let bidder3 = Address::generate(&env);
    
    let hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let hash3 = BytesN::from_array(&env, &[3u8; 32]);
    
    // Commit bids
    client.commit_bid(&tender_id, &bidder1, &hash1);
    client.commit_bid(&tender_id, &bidder2, &hash2);
    client.commit_bid(&tender_id, &bidder3, &hash3);
    
    // Verify all bids
    let bid1 = client.get_bid(&tender_id, &bidder1);
    let bid2 = client.get_bid(&tender_id, &bidder2);
    let bid3 = client.get_bid(&tender_id, &bidder3);
    
    assert_eq!(bid1.commit_hash, hash1);
    assert_eq!(bid2.commit_hash, hash2);
    assert_eq!(bid3.commit_hash, hash3);
}

#[test]
#[should_panic(expected = "Bidding deadline passed")]
fn test_commit_bid_after_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Advance time past deadline
    advance_ledger_time(&env, 1500);
    
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    
    // Should panic - deadline passed
    client.commit_bid(&tender_id, &bidder, &bid_hash);
}

#[test]
fn test_reveal_bid() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Commit bid
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    client.commit_bid(&tender_id, &bidder, &bid_hash);
    
    // Advance time past bidding deadline but before reveal deadline
    advance_ledger_time(&env, 1200);
    
    // Reveal bid
    let bid_amount = 150000i128;
    let secret = String::from_str(&env, "mysecret123");
    
    client.reveal_bid(&tender_id, &bidder, &bid_amount, &secret);
    
    // Verify bid was revealed
    let bid = client.get_bid(&tender_id, &bidder);
    assert_eq!(bid.revealed_amount, Some(bid_amount));
    assert_eq!(bid.is_valid, true);
}

#[test]
#[should_panic(expected = "Reveal phase not started")]
fn test_reveal_bid_too_early() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    client.commit_bid(&tender_id, &bidder, &bid_hash);
    
    // Try to reveal before deadline - should panic
    let bid_amount = 150000i128;
    let secret = String::from_str(&env, "mysecret123");
    
    client.reveal_bid(&tender_id, &bidder, &bid_amount, &secret);
}

#[test]
#[should_panic(expected = "Reveal deadline passed")]
fn test_reveal_bid_too_late() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    client.commit_bid(&tender_id, &bidder, &bid_hash);
    
    // Advance time past reveal deadline
    advance_ledger_time(&env, 2500);
    
    // Try to reveal after deadline - should panic
    let bid_amount = 150000i128;
    let secret = String::from_str(&env, "mysecret123");
    
    client.reveal_bid(&tender_id, &bidder, &bid_amount, &secret);
}

#[test]
#[should_panic(expected = "Bid below minimum")]
fn test_reveal_bid_below_minimum() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    client.commit_bid(&tender_id, &bidder, &bid_hash);
    
    advance_ledger_time(&env, 1200);
    
    // Try to reveal with bid below minimum - should panic
    let bid_amount = 50000i128; // Below min_bid of 100000
    let secret = String::from_str(&env, "mysecret123");
    
    client.reveal_bid(&tender_id, &bidder, &bid_amount, &secret);
}

#[test]
fn test_close_tender_by_admin() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Advance time past reveal deadline
    advance_ledger_time(&env, 2500);
    
    // Close tender as admin
    client.close_tender(&tender_id, &admin);
    
    // Verify tender is closed
    let is_closed = client.is_tender_closed(&tender_id);
    assert_eq!(is_closed, true);
    
    let tender = client.get_tender(&tender_id);
    assert_eq!(tender.is_closed, true);
}

#[test]
fn test_close_tender_by_creator() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Advance time past reveal deadline
    advance_ledger_time(&env, 2500);
    
    // Close tender as creator
    client.close_tender(&tender_id, &creator);
    
    // Verify tender is closed
    let is_closed = client.is_tender_closed(&tender_id);
    assert_eq!(is_closed, true);
}

#[test]
#[should_panic(expected = "Reveal phase not ended")]
fn test_close_tender_too_early() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Try to close before reveal deadline - should panic
    client.close_tender(&tender_id, &admin);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_close_tender_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let unauthorized = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    advance_ledger_time(&env, 2500);
    
    // Try to close as unauthorized user - should panic
    client.close_tender(&tender_id, &unauthorized);
}

#[test]
#[should_panic(expected = "Tender already closed")]
fn test_close_tender_twice() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    advance_ledger_time(&env, 2500);
    
    // Close tender
    client.close_tender(&tender_id, &admin);
    
    // Try to close again - should panic
    client.close_tender(&tender_id, &admin);
}

#[test]
#[should_panic(expected = "Tender is closed")]
fn test_commit_bid_on_closed_tender() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test Project"),
        &String::from_str(&env, "QmHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    advance_ledger_time(&env, 2500);
    client.close_tender(&tender_id, &admin);
    
    // Try to commit bid on closed tender - should panic
    let bid_hash = BytesN::from_array(&env, &[1u8; 32]);
    client.commit_bid(&tender_id, &bidder, &bid_hash);
}

#[test]
fn test_complete_tender_workflow() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    // Step 1: Create tender
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Complete Test Project"),
        &String::from_str(&env, "QmTestHash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    assert_eq!(tender_id, 1);
    
    // Step 2: Multiple bidders commit bids
    let bidder1 = Address::generate(&env);
    let bidder2 = Address::generate(&env);
    let bidder3 = Address::generate(&env);
    
    let hash1 = BytesN::from_array(&env, &[1u8; 32]);
    let hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let hash3 = BytesN::from_array(&env, &[3u8; 32]);
    
    client.commit_bid(&tender_id, &bidder1, &hash1);
    client.commit_bid(&tender_id, &bidder2, &hash2);
    client.commit_bid(&tender_id, &bidder3, &hash3);
    
    // Step 3: Advance time to reveal phase
    advance_ledger_time(&env, 1200);
    
    // Step 4: Bidders reveal their bids
    client.reveal_bid(
        &tender_id,
        &bidder1,
        &150000i128,
        &String::from_str(&env, "secret1"),
    );
    
    client.reveal_bid(
        &tender_id,
        &bidder2,
        &120000i128,
        &String::from_str(&env, "secret2"),
    );
    
    client.reveal_bid(
        &tender_id,
        &bidder3,
        &180000i128,
        &String::from_str(&env, "secret3"),
    );
    
    // Step 5: Verify all bids are valid
    let bid1 = client.get_bid(&tender_id, &bidder1);
    let bid2 = client.get_bid(&tender_id, &bidder2);
    let bid3 = client.get_bid(&tender_id, &bidder3);
    
    assert_eq!(bid1.is_valid, true);
    assert_eq!(bid2.is_valid, true);
    assert_eq!(bid3.is_valid, true);
    
    // Step 6: Advance time past reveal deadline
    advance_ledger_time(&env, 1000);
    
    // Step 7: Close tender
    client.close_tender(&tender_id, &admin);
    
    // Step 8: Verify tender is closed
    let tender = client.get_tender(&tender_id);
    assert_eq!(tender.is_closed, true);
    
    // In a real implementation, the winner would be bidder2 with 120000 (lowest bid)
}

#[test]
fn test_get_tender_count_increments() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    assert_eq!(client.get_tender_count(), 0);
    
    client.create_tender(
        &creator,
        &String::from_str(&env, "Project 1"),
        &String::from_str(&env, "Hash1"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    assert_eq!(client.get_tender_count(), 1);
    
    client.create_tender(
        &creator,
        &String::from_str(&env, "Project 2"),
        &String::from_str(&env, "Hash2"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    assert_eq!(client.get_tender_count(), 2);
    
    client.create_tender(
        &creator,
        &String::from_str(&env, "Project 3"),
        &String::from_str(&env, "Hash3"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    assert_eq!(client.get_tender_count(), 3);
}

#[test]
#[should_panic(expected = "Tender not found")]
fn test_get_nonexistent_tender() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    
    // Try to get tender that doesn't exist
    client.get_tender(&999);
}

#[test]
#[should_panic(expected = "Bid not found")]
fn test_get_nonexistent_bid() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _admin) = create_test_contract(&env);
    let creator = Address::generate(&env);
    let bidder = Address::generate(&env);
    
    let current_time = env.ledger().timestamp();
    let deadline = current_time + 1000;
    let reveal_deadline = deadline + 1000;
    
    let tender_id = client.create_tender(
        &creator,
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "Hash"),
        &deadline,
        &reveal_deadline,
        &100000i128,
    );
    
    // Try to get bid that doesn't exist
    client.get_bid(&tender_id, &bidder);
}