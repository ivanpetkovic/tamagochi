
source "./_config.sh"
INIT='{}'
# secretd tx compute execute $FOOD_ADDRESS "$INIT" --from a --gas 20000000
secretd tx snip20 send $FOOD_ADDRESS $PET_ADDRESS 100 --from a --gas 20000000