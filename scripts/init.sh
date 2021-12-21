FOOD_CONTRACT_ID=1
MARKET_CONTRACT_ID=2
PET_CONTRACT_ID=4


# args: 1 - contract ID, 2 - instance label, 3 - init msg
instantiateContract() {
    secretd tx compute instantiate $1 "$2" --label "$3" --from a -y --keyring-backend test
}

initFoodContract() {
    
    INIT='{"name": "FOOD", "symbol": "FDT", "decimals": 2, "prng_seed": "bmdpbml0ZQ==", "config":{"enable_mint":true, "enable_burn":true, "enable_deposit": true}}'
    # instantiateContract $FOOD_CONTRACT_ID "Food token" '{"name": "FOOD", "symbol": "FDT", "decimals": 2, "prngseed": "nignite"}'
    instantiateContract $FOOD_CONTRACT_ID "$INIT" "Food token" 
    # secretd tx compute instantiate 1 '{"name": "FOOD", "symbol": "FDT", "decimals": 2, "prng_seed": "bmdpbml0ZQ==", "config":{"enable_mint":true, "enable_burn":true, "enable_deposit": true}}' --label "FOOD token" --from a -y --keyring-backend test
}

initMarketContract() {
    INIT='{"token_code_hash": "", "token_address": "", exchange_rate: 0.01}'
    instantiateContract $MARKET_CONTRACT_ID "$INIT" "Market 1" 
}

initPetContract() {
    INIT='{}'
    instantiateContract $PET_CONTRACT_ID "$INIT"  "Pet 1"
}

initFoodContract 
# initMarketContract 
# initPetContract
