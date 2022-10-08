use crate::*;
use mfight_sdk::utils::contract_token_id;
use mfight_sdk::market::MarketOnFtTransferArgs;
use mfight_sdk::ft::FungibleTokenReceiver;
use mfight_sdk::rent::meta::RentOnFtTransferArgs;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Market(MarketOnFtTransferArgs),
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String
    ) -> PromiseOrValue<U128> {
        let ft_token_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_ne!(
            ft_token_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(&sender_id, &signer_id, "owner_id should be signer_id");

        match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
            Args::Market(marketArgs) => {
                self.market.internal_on_ft_transfer(&marketArgs, &ft_token_id, &amount, &sender_id)
            }
        }
    }
}
