source "./_config.sh"
echo $MARKET_ADDRESS
echo $FOOD_ADDRESS
secretd tx compute execute $MARKET_ADDRESS '{"buy_food":{}}' --amount="$1" --from a --gas 20000000