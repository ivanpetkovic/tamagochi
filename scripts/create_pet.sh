source "./_config.sh"
CONTRACT_ID=$1
INIT='{"token_code_hash": "'$FOOD_CODE_HASH'", "token_address": "'$FOOD_ADDRESS'", "satiated_interval": 3, "starving_interval": 1}'

echo  "$INIT"
secretd tx compute instantiate $CONTRACT_ID "$INIT" --label "Pet $1 $2" --from a -y --keyring-backend test
