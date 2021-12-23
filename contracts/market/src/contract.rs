use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State, TokenInfo};
use cosmwasm_std::{
    Api, Binary, Coin, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdResult,
    Storage, Uint128, StdError,
};
use secret_toolkit::snip20;

const TOKEN_DENOM: &str = "uscrt";
/// Ammount of food tokens you can get for 1 SCRT
const DEFAULT_EXCHANGE_RATE: u64 = 100;
const BLOCK_SIZE: usize = 256;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        token: TokenInfo {
            address: HumanAddr::from(msg.token_address),
            code_hash: msg.token_code_hash,
        },
        exchange_rate: msg.exchange_rate.unwrap_or(DEFAULT_EXCHANGE_RATE),
    };
    // market contract should be added as a food token minter
    config(&mut deps.storage).save(&state)?;
    println!("Contract was initialized by {}", env.message.sender);
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::BuyFood {} => try_buy_food(deps, &env),
    }
}

pub fn try_buy_food<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
) -> StdResult<HandleResponse> {
    let sender = &env.message.sender;
    let sent_scrt_funds = env
        .message
        .sent_funds
        .iter()
        .find(|coin: &&Coin| (&coin).denom == TOKEN_DENOM)
        .unwrap()
        .amount;
    if sent_scrt_funds == Uint128::from(0 as u64) {
        return Err(StdError::generic_err("No funds or unsupported tokens sent"));
    }
    let state = config_read(&deps.storage).load()?;
    let food_amount = Uint128::from(sent_scrt_funds.u128() * state.exchange_rate as u128);
    let mint_message = snip20::mint_msg(
        sender.clone(),
        food_amount,
        None,
        BLOCK_SIZE,
        state.token.code_hash.clone(),
        state.token.address.clone(),
    )?;
    Ok(HandleResponse {
        messages: vec![mint_message],
        log: vec![],
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        // QueryMsg::GetFoodBalance {} => to_binary(&query_food_balance(deps)?),
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env};
    // use cosmwasm_std::{coins, from_binary, StdError};

    // #[test]
    // fn proper_initialization() {
    //     let mut deps = mock_dependencies(20, &[]);

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(1000, "earth"));

    //     // we can just call .unwrap() to assert this was a success
    //     let res = init(&mut deps, env, msg).unwrap();
    //     assert_eq!(0, res.messages.len());

    //     // it worked, let's query the state
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(17, value.count);
    // }
}
