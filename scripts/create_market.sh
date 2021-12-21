CONTRACT_ID=$1
TOKEN_ADDRESS="secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg"
TOKEN_CODE_HASH="E6687CD1C4E4ED16712CD7BD4CED08D7E01E7A95E6EA459773BF0C1851F2BA7F"


INIT='{"token_code_hash": "'$TOKEN_CODE_HASH'", "token_address": "'$TOKEN_ADDRESS'", "exchange_rate": 100}'

echo  "Instantiating Market from contract, id=$CONTRACT_ID..."
secretd tx compute instantiate $CONTRACT_ID "$INIT" --label "Market" --from a -y --keyring-backend test
