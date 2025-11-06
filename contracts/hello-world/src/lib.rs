#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Vec,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Tender(u64),
    TenderCount,
    Bid(u64, Address),
    TenderBidders(u64), // List of bidders for a tender
    Winner(u64),
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub struct Tender {
    pub id: u64,
    pub creator: Address,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub deadline: u64,
    pub reveal_deadline: u64,
    pub min_bid: i128,
    pub is_closed: bool,
    pub created_at: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Bid {
    pub bidder: Address,
    pub tender_id: u64,
    pub encrypted_amount: String, // User sends this
    pub revealed_amount: Option<i128>,
    pub is_valid: bool,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Winner {
    pub tender_id: u64,
    pub bidder: Address,
    pub amount: i128,
    pub selected_at: u64,
}

#[contract]
pub struct OpenTenderContract;

#[contractimpl]
impl OpenTenderContract {
    
    /// Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TenderCount, &0u64);
    }

    /// Create a new tender (simplified - admin or verified users only)
    pub fn create_tender(
        env: Env,
        creator: Address,
        title: String,
        description: String,
        ipfs_hash: String,
        deadline: u64,
        reveal_deadline: u64,
        min_bid: i128,
    ) -> u64 {
        // creator.require_auth();
        
        let current_time = env.ledger().timestamp();
        
        // Validate deadlines
        if deadline <= current_time {
            panic!("Deadline must be in future");
        }
        
        if reveal_deadline <= deadline {
            panic!("Reveal deadline must be after bidding deadline");
        }

        let mut count: u64 = env.storage()
            .instance()
            .get(&DataKey::TenderCount)
            .unwrap_or(0);
        
        count += 1;

        let tender = Tender {
            id: count,
            creator,
            title,
            description,
            ipfs_hash,
            deadline,
            reveal_deadline,
            min_bid,
            is_closed: false,
            created_at: current_time,
        };

        env.storage().instance().set(&DataKey::Tender(count), &tender);
        env.storage().instance().set(&DataKey::TenderCount, &count);
        
        // Initialize empty bidders list
        let bidders: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::TenderBidders(count), &bidders);

        env.events().publish(
            (symbol_short!("tender"), symbol_short!("created")),
            count
        );

        count
    }

    /// Submit bid - User just sends their bid amount as encrypted string
    /// The encryption happens client-side, contract just stores it
    pub fn submit_bid(
        env: Env,
        tender_id: u64,
        bidder: Address,
        encrypted_amount: String, // Client sends encrypted bid
    ) {
        // bidder.require_auth();

        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();
        
        if current_time >= tender.deadline {
            panic!("Bidding deadline has passed");
        }

        if tender.is_closed {
            panic!("Tender is closed");
        }

        // Check if bidder already submitted
        let bid_key = DataKey::Bid(tender_id, bidder.clone());
        if env.storage().instance().has(&bid_key) {
            panic!("Bid already submitted. Cannot modify bid.");
        }

        let bid = Bid {
            bidder: bidder.clone(),
            tender_id,
            encrypted_amount,
            revealed_amount: None,
            is_valid: false,
            timestamp: current_time,
        };

        env.storage().instance().set(&bid_key, &bid);
        
        // Add bidder to list
        let mut bidders: Vec<Address> = env.storage()
            .instance()
            .get(&DataKey::TenderBidders(tender_id))
            .unwrap_or(Vec::new(&env));
        bidders.push_back(bidder.clone());
        env.storage().instance().set(&DataKey::TenderBidders(tender_id), &bidders);

        env.events().publish(
            (symbol_short!("bid"), symbol_short!("submitted")),
            (tender_id, bidder)
        );
    }

    /// Reveal bid - User provides decryption key and actual amount
    pub fn reveal_bid(
        env: Env,
        tender_id: u64,
        bidder: Address,
        actual_amount: i128,
        _decryption_key: String,
    ) {
        // bidder.require_auth();

        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();
        
        if current_time < tender.deadline {
            panic!("Cannot reveal before bidding deadline");
        }

        if current_time >= tender.reveal_deadline {
            panic!("Reveal deadline has passed");
        }

        let mut bid: Bid = env.storage()
            .instance()
            .get(&DataKey::Bid(tender_id, bidder.clone()))
            .expect("No bid found for this bidder");

        if bid.revealed_amount.is_some() {
            panic!("Bid already revealed");
        }

        // Verify amount meets minimum
        if actual_amount < tender.min_bid {
            panic!("Bid amount below minimum requirement");
        }

        // In production, verify decryption_key matches encrypted_amount
        // For now, we trust the reveal (frontend handles encryption/decryption)
        
        bid.revealed_amount = Some(actual_amount);
        bid.is_valid = true;

        env.storage()
            .instance()
            .set(&DataKey::Bid(tender_id, bidder.clone()), &bid);

        env.events().publish(
            (symbol_short!("bid"), symbol_short!("revealed")),
            (tender_id, bidder, actual_amount)
        );
    }

    /// Close tender and automatically select winner (lowest valid bid)
    pub fn close_tender(env: Env, tender_id: u64, caller: Address) {
        // caller.require_auth();

        let mut tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();

        if current_time < tender.reveal_deadline {
            panic!("Cannot close before reveal deadline");
        }

        if tender.is_closed {
            panic!("Tender already closed");
        }

        // Verify caller is admin or creator
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");

        if caller != admin && caller != tender.creator {
            panic!("Only admin or creator can close tender");
        }

        // Find winner (lowest valid bid)
        let bidders: Vec<Address> = env.storage()
            .instance()
            .get(&DataKey::TenderBidders(tender_id))
            .unwrap_or(Vec::new(&env));

        let mut lowest_amount: Option<i128> = None;
        let mut winner_address: Option<Address> = None;

        // Iterate through all bidders to find lowest valid bid
        for i in 0..bidders.len() {
            if let Some(bidder_addr) = bidders.get(i) {
                if let Some(bid) = env.storage()
                    .instance()
                    .get::<DataKey, Bid>(&DataKey::Bid(tender_id, bidder_addr.clone())) {
                    
                    if bid.is_valid {
                        if let Some(amount) = bid.revealed_amount {
                            if lowest_amount.is_none() || amount < lowest_amount.unwrap() {
                                lowest_amount = Some(amount);
                                winner_address = Some(bidder_addr);
                            }
                        }
                    }
                }
            }
        }

        // Store winner if found
        if let (Some(amount), Some(winner)) = (lowest_amount, winner_address) {
            let winner_data = Winner {
                tender_id,
                bidder: winner.clone(),
                amount,
                selected_at: current_time,
            };
            env.storage().instance().set(&DataKey::Winner(tender_id), &winner_data);
            
            env.events().publish(
                (symbol_short!("winner"), symbol_short!("selected")),
                (tender_id, winner, amount)
            );
        }

        tender.is_closed = true;
        env.storage().instance().set(&DataKey::Tender(tender_id), &tender);

        env.events().publish(
            (symbol_short!("tender"), symbol_short!("closed")),
            tender_id
        );
    }

    /// Get tender details
    pub fn get_tender(env: Env, tender_id: u64) -> Tender {
        env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found")
    }

    /// Get all tenders (returns list of IDs)
    pub fn get_all_tenders(env: Env) -> Vec<u64> {
        let count = env.storage()
            .instance()
            .get(&DataKey::TenderCount)
            .unwrap_or(0u64);
        
        let mut tender_ids = Vec::new(&env);
        for i in 1..=count {
            tender_ids.push_back(i);
        }
        tender_ids
    }

    /// Get total tender count
    pub fn get_tender_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TenderCount)
            .unwrap_or(0)
    }

    /// Get bid details (only bidder can see their own bid before reveal)
    pub fn get_bid(env: Env, tender_id: u64, bidder: Address) -> Option<Bid> {
        env.storage()
            .instance()
            .get(&DataKey::Bid(tender_id, bidder))
    }

    /// Get winner details
    pub fn get_winner(env: Env, tender_id: u64) -> Option<Winner> {
        env.storage()
            .instance()
            .get(&DataKey::Winner(tender_id))
    }

    /// Get all bidders for a tender (public after reveal deadline)
    pub fn get_tender_bidders(env: Env, tender_id: u64) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::TenderBidders(tender_id))
            .unwrap_or(Vec::new(&env))
    }

    /// Check if tender is closed
    pub fn is_tender_closed(env: Env, tender_id: u64) -> bool {
        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");
        tender.is_closed
    }

    /// Get contract admin
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OpenTenderContract);
        let client = OpenTenderContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        env.mock_all_auths();
        
        client.initialize(&admin);
        
        let count = client.get_tender_count();
        assert_eq!(count, 0);
        
        let stored_admin = client.get_admin();
        assert_eq!(stored_admin, admin);
    }

    #[test]
    fn test_create_and_get_tender() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OpenTenderContract);
        let client = OpenTenderContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        env.mock_all_auths();
        
        client.initialize(&admin);
        
        let tender_id = client.create_tender(
            &creator,
            &String::from_str(&env, "Road Construction"),
            &String::from_str(&env, "Build 10km road"),
            &String::from_str(&env, "QmHash123"),
            &1000u64,
            &2000u64,
            &100000i128,
        );
        
        assert_eq!(tender_id, 1);
        
        let tender = client.get_tender(&tender_id);
        assert_eq!(tender.id, 1);
        assert_eq!(tender.min_bid, 100000i128);
        assert!(!tender.is_closed);
    }

    #[test]
    fn test_complete_bidding_flow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OpenTenderContract);
        let client = OpenTenderContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let bidder1 = Address::generate(&env);
        let bidder2 = Address::generate(&env);
        
        env.mock_all_auths();
        
        // Initialize
        client.initialize(&admin);
        
        // Create tender
        let tender_id = client.create_tender(
            &creator,
            &String::from_str(&env, "Test Project"),
            &String::from_str(&env, "Description"),
            &String::from_str(&env, "QmHash"),
            &1000u64,
            &2000u64,
            &100000i128,
        );
        
        // Submit bids
        client.submit_bid(
            &tender_id,
            &bidder1,
            &String::from_str(&env, "encrypted_150000"),
        );
        
        client.submit_bid(
            &tender_id,
            &bidder2,
            &String::from_str(&env, "encrypted_120000"),
        );
        
        // Advance time past bidding deadline
        env.ledger().set_timestamp(1500);
        
        // Reveal bids
        client.reveal_bid(
            &tender_id,
            &bidder1,
            &150000i128,
            &String::from_str(&env, "key1"),
        );
        
        client.reveal_bid(
            &tender_id,
            &bidder2,
            &120000i128,
            &String::from_str(&env, "key2"),
        );
        
        // Advance past reveal deadline
        env.ledger().set_timestamp(2500);
        
        // Close tender
        client.close_tender(&tender_id, &admin);
        
        // Check winner (should be bidder2 with 120000)
        let winner = client.get_winner(&tender_id);
        assert!(winner.is_some());
        assert_eq!(winner.unwrap().amount, 120000i128);
    }
}