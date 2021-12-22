source "./_config.sh"
CONTRACT_ID=$1
INIT='{"token_code_hash": "'$FOOD_CODE_HASH'", "token_address": "'$FOOD_ADDRESS'", "exchange_rate": 100}'

echo  "Instantiating Market from contract, id=$CONTRACT_ID..."
secretd tx compute instantiate $CONTRACT_ID "$INIT" --label "Market $1 $2" --from a -y --keyring-backend test
