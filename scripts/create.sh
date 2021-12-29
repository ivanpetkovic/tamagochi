source "./_config.sh"
echo $PET_ADDRESS
secretd tx compute execute $PET_ADDRESS '{"create":{"name": "'$1'"}}' --from a --gas 20000000