// Find all our documentation at https://docs.near.org
use near_sdk::{env, near, NearToken, Promise, PublicKey};
use serde::Serialize;
use serde_json::Value;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    beneficiary_public_key: Option<PublicKey>,
    beneficiary_created_date: Option<u64>,
    beneficiary_updated_date: Option<u64>,
    beneficiary_effective_date: Option<u64>,
}



// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiary_public_key: None,
            beneficiary_created_date: None,
            beneficiary_updated_date: None,
            beneficiary_effective_date: None,
        }
    }
}

#[derive(Serialize)]
struct Will {
    beneficiary_public_key: Option<PublicKey>,
    beneficiary_created_date: Option<u64>,
    beneficiary_updated_date: Option<u64>,
    beneficiary_effective_date: Option<u64>,
}

// Implement the contract structure
#[near]
impl Contract {
    #[private]
    #[payable]
    pub fn create_will(&mut self, beneficiary_public_key: PublicKey, beneficiary_effective_date: u64) {
        assert!(self.beneficiary_public_key.is_none(), "Will has already been created, delete the old will if you want to create a new will");
        assert!(env::attached_deposit() >= NearToken::from_yoctonear(1), "Attached deposit must be at least 1 yoctoNEAR");

        self.beneficiary_public_key = Some(beneficiary_public_key.clone());
        self.beneficiary_created_date = Some(env::block_timestamp());
        self.beneficiary_updated_date = Some(env::block_timestamp());
        self.beneficiary_effective_date = Some(beneficiary_effective_date);
    }

    #[private]
    #[payable]
    pub fn delete_will(&mut self) {
        assert!(env::attached_deposit() >= NearToken::from_yoctonear(1), "Attached deposit must be at least 1 yoctoNEAR");

        self.beneficiary_public_key = None;
        self.beneficiary_created_date = None;
        self.beneficiary_updated_date = None;
        self.beneficiary_effective_date = None;
    }

    #[private]
    #[payable]
    pub fn extend_will(&mut self, beneficiary_effective_date: u64) {
        assert!(env::attached_deposit() >= NearToken::from_yoctonear(1), "Attached deposit must be at least 1 yoctoNEAR");
        assert!(self.beneficiary_effective_date.is_some(), "Beneficiary effective date has not been set");
        assert!(self.beneficiary_effective_date.unwrap() < beneficiary_effective_date, "Beneficiary effective date must be later than the current effective date.\nIf you want to shorten the will time, delete the old will and create a new one.");

        self.beneficiary_updated_date = Some(env::block_timestamp());
        self.beneficiary_effective_date = Some(beneficiary_effective_date);
    }

    pub fn execute_will(&mut self) -> Promise {
        assert!(self.beneficiary_public_key.is_some(), "Will has not been created");
        assert!(self.beneficiary_effective_date.is_some(), "Beneficiary effective date has not been set");
        assert!(env::block_timestamp() >= self.beneficiary_effective_date.unwrap(), "Beneficiary effective date has not been reached");

        let beneficiary_public_key = self.beneficiary_public_key.clone();

        self.beneficiary_public_key = None;
        self.beneficiary_created_date = None;
        self.beneficiary_updated_date = None;
        self.beneficiary_effective_date = None;

        // What if this promise failed? The beneficiary public key already deleted
        Promise::new(env::current_account_id()).add_full_access_key(beneficiary_public_key.unwrap())
    }

    pub fn get_will(&self) -> Value {
        let will = Will {
            beneficiary_public_key: self.beneficiary_public_key.clone(),
            beneficiary_created_date: self.beneficiary_created_date,
            beneficiary_updated_date: self.beneficiary_updated_date,
            beneficiary_effective_date: self.beneficiary_effective_date,
        };

        serde_json::to_value(&will).unwrap()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn default_will_is_empty() {
        let contract = Contract::default();

        assert_eq!(contract.beneficiary_public_key, None);
        assert_eq!(contract.beneficiary_created_date, None);
    }
}
