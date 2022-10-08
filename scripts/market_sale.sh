#!/bin/bash
source neardev/dev-account.env
TOKEN_ID="1958"
near view $CONTRACT_NAME market_sale "{ \"contract_id\": \"$NFT_CONTRACT\", \"token_id\": \"$TOKEN_ID\" }"
