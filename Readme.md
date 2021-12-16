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

Or use the Secret Network's optimizer for each contract manually. (if the above does not work)

The optimized contracts are generated in the artifacts/ directory.

## Using the contracts

**Note: section is a WIP**
_Make sure to upload all contracts first_

1. Create an instance of the Food contract using the following init message:

```javascript
{
   "name":"Food",
   "symbol":"FDT",
   "decimals":2, // for a conversion of 1/100
   "prng_seed":<random_string>,
   "config":{
      "enable_mint":true, //to be used from the market
      "enable_burn":true
   }
}
```

2. Create an instance of the Market contract using the following init message:
   _TODO_
3. Create an instance of the Pet contract
    _TODO_

## Playing rules

_TODO_
