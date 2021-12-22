source "./_config.sh"
# secretd tx compute execute $FOOD_ADDRESS '{"balance": {"address": "'$USER_ADDRES'", ""}}' --from a --gas 2000000
secretd q snip20 balance $FOOD_ADDRESS $USER_ADDRES $VIEWING_KEY