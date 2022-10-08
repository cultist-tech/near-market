#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME market_supply_by_nft_contract_id "{ \"nft_contract_id\": \"$NFT_CONTRACT\", \"from_index\": \"0\", \"limit\": 10000 }"
