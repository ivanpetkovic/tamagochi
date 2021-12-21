echo "Current folder: $(pwd)"


docker run -it --rm \
 -p 26657:26657 -p 26656:26656 -p 1337:1337 \
 -v $(pwd):/root/code \
 --name secretdev enigmampc/secret-network-sw-dev:v1.2.0


# secretcli tx compute store contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test