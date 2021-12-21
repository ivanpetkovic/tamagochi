CONTRACT_ID=1
INIT='{"name": "FOOD", "symbol": "FDT", "decimals": 2, "prng_seed": "bmdpbml0ZQ==", "config":{"enable_mint":true, "enable_burn":true, "enable_deposit": true}}'

secretd tx compute instantiate $CONTRACT_ID "$INIT" --label "Food token" --from a -y --keyring-backend test
