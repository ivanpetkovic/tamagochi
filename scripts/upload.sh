echo "Uploading $1 ..."
secretd tx compute store $1 --from a --gas 20000000 -y --keyring-backend test
echo "Done..."
