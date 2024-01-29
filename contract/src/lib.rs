mod eth;

use eth::EthAddress;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::assert_one_yocto;
use near_sdk::store::LookupSet;
use near_sdk::{env, near_bindgen, AccountId, Gas, PanicOnDefault, Promise};
use serde_json::json;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    mpc_account_id: AccountId,
    total_wallets: u64,
    wallet_owners: UnorderedMap<String, LookupSet<AccountId>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(mpc_account_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            mpc_account_id: mpc_account_id,
            total_wallets: 0,
            wallet_owners: UnorderedMap::new(b"w"),
        }
    }

    #[payable]
    pub fn create_wallet(&mut self) {
        assert_one_yocto();
        let path = format!("m/44'/397'/0'/0'/{}", self.total_wallets);

        let mut key = b"o".to_vec();
        key.extend_from_slice(path.as_bytes());

        let mut owners = LookupSet::new(key);
        owners.insert(env::predecessor_account_id());
        self.wallet_owners.insert(&path, &owners);

        self.total_wallets += 1;
        env::log_str(format!("New wallet: {path}").as_str());
        env::log_str(
            format!(
                "Assess to wallet {path} has been granted to {}",
                env::predecessor_account_id()
            )
            .as_str(),
        );
    }

    #[payable]
    pub fn sign(&mut self, payload: [u8; 32], wallet_id: String) -> Promise {

        // self.assert_wallet_owner(&wallet_id);

        env::log_str(
            format!(
                "Transaction from {wallet_id} has been signed by {}",
                env::predecessor_account_id()
            )
            .as_str(),
        );

        Promise::new(self.mpc_account_id.clone()).function_call(
            "sign".to_string(),
            json!({
                "path": wallet_id,
                "payload": payload,
            })
            .to_string()
            .into_bytes(),
            0,
            Gas(50_000_000_000_000),
        )
    }

    fn assert_wallet_owner(&self, wallet_id: &String) {
        let wallet_owners = self.wallet_owners.get(wallet_id);
        assert!(wallet_owners.is_none(), "Wallet not found");
        assert_one_yocto();
        assert!(
            wallet_owners.unwrap()
                .contains(&env::predecessor_account_id()),
            "Predecessor is not owner of this wallet"
        );
    }

    #[payable]
    pub fn grant_assess(&mut self, wallet_id: String, account_id: AccountId) {
        self.assert_wallet_owner(&wallet_id);

        env::log_str(
            format!("Assess to wallet {wallet_id} has been granted to {account_id}").as_str(),
        );

        self.wallet_owners
            .get(&wallet_id)
            .unwrap()
            .insert(account_id);
    }

    #[payable]
    pub fn revoke_assess(&mut self, wallet_id: String, account_id: AccountId) {
        self.assert_wallet_owner(&wallet_id);
        env::log_str(
            format!("Assess to wallet {wallet_id} has been revoked from {account_id}").as_str(),
        );

        self.wallet_owners
            .get(&wallet_id)
            .unwrap()
            .remove(&account_id);
    }

    // ETH Api
    #[payable]
    pub fn eth_transfer_call(&mut self, wallet_id: String, recipient: String, amount: u64, nonce: u64, gas_price: u64, gas_limit: u64, data: Vec<u8>) -> Promise {
        // self.assert_wallet_owner(&wallet_id);

        let transaction = eth::EthereumTransaction::new(
            nonce,           
            gas_price,  
            gas_limit,
            EthAddress::from(recipient.as_str()).unwrap(),  
            amount,  
            vec![],      
        );
    
        let tx_hash = transaction.calculate_hash();
        env::log_str(format!("Transaction hash: {:?}", tx_hash).as_str());
    }

    pub fn get_wallets_in_range(&self, from: usize, to: usize) -> Vec<String> {
        let wallets: Vec<String> = self.wallet_owners.keys().collect();
        wallets.into_iter().skip(from).take(to - from).collect()
    }

}
