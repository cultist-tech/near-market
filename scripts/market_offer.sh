#!/bin/bash
source neardev/dev-account.env

TOKEN_ID="121685"
ACCOUNT_ID="muzikant.testnet"

near call $CONTRACT_NAME market_offer --accountId $ACCOUNT_ID "{ \"nft_contract_id\": \"$NFT_CONTRACT\", \"token_id\": \"$TOKEN_ID\" }" --deposit "1" --gas "200000000000000"
