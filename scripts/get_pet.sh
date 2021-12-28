source "./_config.sh"
echo $PET_ADDRESS
secretd query compute query $PET_ADDRESS '{"pet": {"address":"'$USER_ADDRES'", "viewing_key": "'$VIEWING_KEY'"}}' 
