source "./_config.sh"
echo $PET_ADDRESS
secretd tx compute execute $PET_ADDRESS '{"pet":{}}' --from a --gas 20000000