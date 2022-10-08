#!/bin/bash
source neardev/dev-account.env

near call $CONTRACT_NAME market_add_ft_token --accountId $CONTRACT_NAME "{ \"nft_contract_id\": \"$FT_CONTRACT\" }"
