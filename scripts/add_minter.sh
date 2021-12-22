source "./_config.sh"
echo "FOOD_ADR:$FOOD_ADDRESS, MARKET_ADR:$MARKET_ADDRESS"
secretd tx compute execute $FOOD_ADDRESS '{"add_minters": {"minters":["'$MARKET_ADDRESS'"]} }' --from a --gas 20000000
