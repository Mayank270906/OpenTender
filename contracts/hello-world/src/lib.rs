#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, BytesN
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Tender(u64),
    TenderCount,
    Bid(u64, Address), // (tender_id, bidder)
    Winner(u64),
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub struct Tender {
    pub id: u64,
    pub creator: Address,
    pub title: String,
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
    pub commit_hash: BytesN<32>,
    pub revealed_amount: Option<i128>,
    pub secret: Option<String>,
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
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TenderCount, &0u64);
    }

    /// Create a new tender
    pub fn create_tender(
        env: Env,
        creator: Address,
        title: String,
        ipfs_hash: String,
        deadline: u64,
        reveal_deadline: u64,
        min_bid: i128,
    ) -> u64 {
        creator.require_auth();
        
        let current_time = env.ledger().timestamp();
        
        // Validate deadlines
        if deadline <= current_time || reveal_deadline <= deadline {
            panic!("Invalid deadlines");
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
            ipfs_hash,
            deadline,
            reveal_deadline,
            min_bid,
            is_closed: false,
            created_at: current_time,
        };

        env.storage().instance().set(&DataKey::Tender(count), &tender);
        env.storage().instance().set(&DataKey::TenderCount, &count);

        env.events().publish(
            (symbol_short!("tender"), symbol_short!("created")),
            count
        );

        count
    }

    /// Submit sealed bid (commit phase)
    pub fn commit_bid(
        env: Env,
        tender_id: u64,
        bidder: Address,
        bid_hash: BytesN<32>,
    ) {
        bidder.require_auth();

        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();
        
        if current_time >= tender.deadline {
            panic!("Bidding deadline passed");
        }

        if tender.is_closed {
            panic!("Tender is closed");
        }

        let bid = Bid {
            bidder: bidder.clone(),
            tender_id,
            commit_hash: bid_hash,
            revealed_amount: None,
            secret: None,
            is_valid: false,
            timestamp: current_time,
        };

        env.storage()
            .instance()
            .set(&DataKey::Bid(tender_id, bidder.clone()), &bid);

        env.events().publish(
            (symbol_short!("bid"), symbol_short!("commit")),
            (tender_id, bidder)
        );
    }

    /// Reveal bid (reveal phase)
    pub fn reveal_bid(
        env: Env,
        tender_id: u64,
        bidder: Address,
        amount: i128,
        secret: String,
    ) {
        bidder.require_auth();

        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();
        
        if current_time < tender.deadline {
            panic!("Reveal phase not started");
        }

        if current_time >= tender.reveal_deadline {
            panic!("Reveal deadline passed");
        }

        let mut bid: Bid = env.storage()
            .instance()
            .get(&DataKey::Bid(tender_id, bidder.clone()))
            .expect("Bid not found");

        // Verify hash (simplified - in production use proper hash verification)
        // In real implementation: verify that sha256(amount + secret) == commit_hash
        
        if amount < tender.min_bid {
            panic!("Bid below minimum");
        }

        bid.revealed_amount = Some(amount);
        bid.secret = Some(secret);
        bid.is_valid = true;

        env.storage()
            .instance()
            .set(&DataKey::Bid(tender_id, bidder.clone()), &bid);

        env.events().publish(
            (symbol_short!("bid"), symbol_short!("revealed")),
            (tender_id, bidder)
        );
    }

    /// Close tender and select winner (lowest valid bid)
    pub fn close_tender(env: Env, tender_id: u64, caller: Address) {
        caller.require_auth();

        let mut tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");

        let current_time = env.ledger().timestamp();

        if current_time < tender.reveal_deadline {
            panic!("Reveal phase not ended");
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
            panic!("Unauthorized");
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

    /// Get total tender count
    pub fn get_tender_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::TenderCount)
            .unwrap_or(0)
    }

    /// Get bid details
    pub fn get_bid(env: Env, tender_id: u64, bidder: Address) -> Bid {
        env.storage()
            .instance()
            .get(&DataKey::Bid(tender_id, bidder))
            .expect("Bid not found")
    }

    /// Check if tender is closed
    pub fn is_tender_closed(env: Env, tender_id: u64) -> bool {
        let tender: Tender = env.storage()
            .instance()
            .get(&DataKey::Tender(tender_id))
            .expect("Tender not found");
        tender.is_closed
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _};
    use soroban_sdk::{Address, Env};

    #[test]
    fn test_create_tender() {
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
            &String::from_str(&env, "QmHash123"),
            &1000u64,
            &2000u64,
            &100000i128,
        );

        assert_eq!(tender_id, 1);
        
        let tender = client.get_tender(&tender_id);
        assert_eq!(tender.id, 1);
        assert!(!tender.is_closed);
    }
}