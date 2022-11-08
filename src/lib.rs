use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::json_types::{ U128 };
use near_sdk::{
    env,
    ext_contract,
    near_bindgen,
    AccountId,
    PanicOnDefault,
    Promise,
    PromiseOrValue,
    CryptoHash,
    BorshStorageKey,
    assert_self,
};
use mfight_sdk::market::base::MarketFeature;
use mfight_sdk::market::{  TokenId };
use mfight_sdk::metadata::FungibleTokenId;
use mfight_sdk::reputation::ReputationFeature;
use mfight_sdk::reputation::BUY_INCREMENT;

mod ft_callbacks;
mod nft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub market: MarketFeature,
    pub reputation: ReputationFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    ByOwnerId,
    ByNFTContractId,
    FTTokenIds,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_with_default_meta(owner_id: AccountId) -> Self {
        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id: owner_id.clone().into(),
            market: MarketFeature::new(
                owner_id.clone(),
                None,
                None,
                StorageKey::Sales,
                StorageKey::ByOwnerId,
                StorageKey::ByNFTContractId,
                StorageKey::FTTokenIds,
                StorageKey::StorageDeposits
            ),
            reputation: ReputationFeature::new()
        };

        this
    }
    
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        #[derive(BorshDeserialize)]
        struct Old {            
            owner_id: AccountId,
            market: MarketFeature,            
        }

        let old: Old = env::state_read().expect("Error");

        Self {
            owner_id: old.owner_id,
            market: old.market,
            reputation: ReputationFeature::new(),
        }
    }

}

mfight_sdk::impl_market_core!(Contract, market);
mfight_sdk::impl_market_enumeration!(Contract, market);
mfight_sdk::impl_reputation_feature!(Contract, reputation);
