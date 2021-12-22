# Tamagotchi

The traditional Japanese digital pet. Recreated using smart contracts.

## Challenge

Fast forward 20 years. Lets implement a basic Tamagochi logic.

- Create a SNIP-20 token contract - called `FOOD`.
- Create a CosmWasm `Market` contract that gives the ability to the user to purchase SNIP-20 tokens to feed to their Pet.
- Create a CosmWasm `Pet` contract which implements a digital pet (Tamagochi like) interface, allowing one to feed the creature with SNIP-20/21/22/23 tokens.

## Requirements / restrictions

- `FOOD` tokens can only be minted by the Market contract.
- `FOOD` tokens can be purchased only with the native SCRT token in ratio `SCRT/FOOD` - `1:100`.
- Feeding the `Pet` (sending the `Pet` contract a `FOOD` token) should result in burning `FOOD` tokens.
- `Pet` should implement a countdown clock, which gets pushed `4` hours ahead every time the pet gets fed.
- If you miss feeding the `Pet` for more than 4 hours then the `Pet` will starve.
- If a `Pet` starves to death then it cannot be fed any longer so `FOOD` tokens shall be returned to the sender rather then burned.


## References
- [scrt.network](https://scrt.network)
- https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-20.md
- cosmwasm.com
- github.com/hackbg/fadroma


## Contracts

| Name                         | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| [`Market`](contracts/Market) | Used as a marketplace to buy FOOD tokens       |
| [`Food`](packages/Food)      | Snip-20 contract, used to create the token     |
| [`Pet`](contracts/Pet)       | Tamagotchi like interface through the contract |


# Idea

## Buying Food tokens

1. User sends SCRT to Market contract (Handle::BuyFood)
2. Market contract will add given SCRT to the overall balance, and deduct them from the user's balance.
3. Market's response contains a message to Food contract to mint ceratain amount of tokens to the user's address

## Feeding a pet

1. Users sends Food tokens via Food contract to Pet contract
2. Pet contract will check the user's balance using the supplied view key. Abort if insufficent food for feeding (1 feeding = 100 Food)
3. Pet contract will try to feed the pet. If not possible, abort.
4. If pet is fed, send message to the Food contract to burn the user's Food tokens. If user sent more than 100 tokens, burn them anyways

# Usage

## Building the contracts

To start building run:

```
cargo build --release --target wasm32-unknown-unknown && cargo schema
```

This will build all the binaries and generate the schema.

For a production ready (optimized & compressed) build, run the following from the root of the repo:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.3
```

Or use the Secret Network's optimizer for each contract manually. (if the above does not work). You can use script optimize.sh to do that \
for each contract. Keep in mind that workspace atifacts, like top-level Cargo.toml and target folder must be deleted. This will in turn disable \ 
rust-analyzer plugin in VCS, used in dev. 

Make sure to build contract after removing above mentioned items.

## Starting the local test blockchain

You can start local blockchain inside docker using

```
   ./scripts/start.sh
```
Open a separate terminal and enter the container using the 

```
   docker exec -it secretdev /bin/bash
   cd code/scripts
```

## Setting up the contracts

1. Make sure to upload all contracts first by using the upload.sh script. Provided you are inside the container at code/scripts, run:
```
   ./upload.sh ../package/food/contract.wasm.gz
   ./upload.sh ../contract/market/contract.wasm.gz
   ./upload.sh ../contract/pet/contract.wasm.gz
```

2. Create an instance of the Food contract using the following init message:

```
   ./scripts/create_food.sh <food_code_id>
```
3. Edit ./scripts/_config.sh and enter food contract's address and code hash
4. Create an instance of the Market contract suppying a Food contract as a token contract,  using the following init script:

```
   ./scripts/create_market.sh <market_code_id>
```
5. Edit ./scripts/_config.sh and enter market contract's address
6. Add Market contract as minter for Food contract

```
   ./scripts/add_minter.sh
```

7. Create an instance of the Pet contract suppying a Food contract as a token contract, using the following init script:

```
   ./scripts/create_pet.sh <pet_code_id>
```
If you want to alter satiation and starvatrion period, you can edit the script
8. Edit ./scripts/_config.sh and enter pet contract's address
9. Edit ./scripts/_config.sh and enter your wallet's address

## Interacting with the dapp


You can buy Food tokens using the following script (provided you are inside the container at code/scripts/)

```
   ./buy_food.sh 100uscrt
```
which will buy 100 food tokens for each uscrt

You can feed the Tamagochi, only when it's hungry, and before it starves to death.
```
   ./feed_pet.sh
```
If you want to check you balance, create a viewing key first, and then run the script:
```
   ./create_viewing_key.sh
   ./query_balance.sh
```