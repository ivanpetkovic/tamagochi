  for WASM in ./artifacts/*.wasm; do
    echo "Uploading $WASM ..."
    secretd tx compute store $WASM --from a --gas 20000000 -y --keyring-backend test
    echo "Done..."
    read -p "Press any key for continue"
  done
