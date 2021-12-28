source "./_config.sh"
echo $PET_ADDRESS
secretd tx compute execute $PET_ADDRESS '{"set_name":{"name": "'$1'"}}' --from a --gas 20000000