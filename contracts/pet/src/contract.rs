use secret_toolkit::snip20;
use std::ops::Add;
use std::time::Duration;

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, InitMsg, Minutes, QueryMsg};
use crate::state::{pet, pet_read, State, TokenInfo};

const BLOCK_SIZE: usize = 256;
const DEFAULT_SATIATED_TIME: Minutes = 180;
const DEFAULT_STARVING_TIME: Minutes = 60;
const TOKENS_PER_FEEDING: u16 = 100;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        last_feed_time: env.block.time,
        satiated_interval: msg.satiated_interval.unwrap_or(DEFAULT_SATIATED_TIME),
        starving_interval: msg.starving_interval.unwrap_or(DEFAULT_STARVING_TIME),
        owner: deps.api.canonical_address(&env.message.sender)?,
        token_info: TokenInfo {
            address: HumanAddr(msg.token_address.clone()),
            code_hash: msg.token_code_hash.clone(),
        },
    };

    pet(&mut deps.storage).save(&state)?;

    println!("Pet was born and fed, thanks to {}", env.message.sender);
    let pet_contract_hash = &env.contract_code_hash;
    let callback = snip20::register_receive_msg(
        pet_contract_hash.clone(),
        None,
        BLOCK_SIZE,
        msg.token_code_hash.clone(),
        HumanAddr(msg.token_address.clone()),
    )?;

    Ok(InitResponse {
        messages: vec![callback],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Receive {
            sender,
            from,
            amount,
            ..
        } => try_feed(&mut deps.storage, &env, sender, amount),
    }
}

pub fn try_feed<S: Storage>(
    storage: &mut S,
    env: &Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let time = env.block.time;
    let state = &pet_read(storage).load()?;
    if is_dead(state, time) {
        return Err(StdError::generic_err("Pet is dead :("));
    }
    if !is_hungry(state, time) {
        return Err(StdError::generic_err("Pet is not hungry"));
    }
    if amount < Uint128(TOKENS_PER_FEEDING as u128) {
        return Err(StdError::generic_err(
            "You need more tokens to feed the pet",
        ));
    }
    let _ = pet(storage).update(|mut state| {
        state.last_feed_time = time;
        Ok(state)
    })?;
    let burn_msg = snip20::burn_msg(
        amount,
        None,
        BLOCK_SIZE,
        state.token_info.code_hash,
        state.token_info.address,
    );
    Ok(HandleResponse {
        messages: vec![burn_msg],
        log: vec![log("current_time", time), log("is_hungry", time)],
        data: None,
    })
}

fn to_seconds(interval: Minutes) -> u64 {
    (interval * 60) as u64
}

fn is_dead(state: &State, current_time: u64) -> bool {
    state.last_feed_time + to_seconds(state.satiated_interval) + to_seconds(state.starving_interval)
        < current_time
}

fn is_hungry(state: &State, current_time: u64) -> bool {
    state.last_feed_time + to_seconds(state.satiated_interval) < current_time
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    // match msg {
    //     QueryMsg::CanEat {} => to_binary(&can_eat(deps)?),
    //     QueryMsg::IsHungry {} => to_binary( &is_hungry(deps)?)
    // }
    Ok(to_binary("test")?)
}

// fn can_eat<S:Storage, A:Api, Q:Querier>(deps: &Extern<S, A, Q>) -> StdResult<bool> {
//     Ok(true)
// }

// fn is_hungry<S:Storage, A:Api, Q:Querier>(deps: &Extern<S, A, Q>) -> StdResult<bool> {
//     let state = pet_read(&deps.storage).load()?;
//     // deps.
//     // state.last_feed_time
//     Ok(true)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

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

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, env, msg).unwrap();

    //     // anyone can increment
    //     let env = mock_env("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Increment {};
    //     let _res = handle(&mut deps, env, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let msg = InitMsg { count: 17 };
    //     let env = mock_env("creator", &coins(2, "token"));
    //     let _res = init(&mut deps, env, msg).unwrap();

    //     // not anyone can reset
    //     let unauth_env = mock_env("anyone", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let res = handle(&mut deps, unauth_env, msg);
    //     match res {
    //         Err(StdError::Unauthorized { .. }) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_env = mock_env("creator", &coins(2, "token"));
    //     let msg = HandleMsg::Reset { count: 5 };
    //     let _res = handle(&mut deps, auth_env, msg).unwrap();

    //     // should now be 5
    //     let res = query(&deps, QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
