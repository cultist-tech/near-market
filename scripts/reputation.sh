#!/bin/bash
source neardev/dev-account.env
near view $CONTRACT_NAME reputation "{ \"account_id\": \"muzikant.testnet\" }"
