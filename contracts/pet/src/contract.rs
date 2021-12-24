use secret_toolkit::snip20;

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleMsg, InitMsg, Minutes, QueryMsg, ResponseStatus, HandleAnswer};
use crate::state::{config_read, Pet, Pets, State, TokenInfo};

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
        owner: deps.api.canonical_address(&env.message.sender)?,
        token_info: TokenInfo {
            address: HumanAddr(msg.token_address.clone()),
            code_hash: msg.token_code_hash.clone(),
        },
    };

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
        } => try_feed(deps, &env, sender, amount),
        HandleMsg::SetName { name } => try_set_name(deps, &env, name),
        HandleMsg::GetName {} => try_get_name(deps, env),
    }
}

fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let time = env.block.time;
    let state = config_read(&deps.storage).load()?;
    let mut pets = Pets::from_storage(&mut deps.storage);
    let mut pet: Pet;
    let user_address = deps.api.canonical_address(&sender)?;
    match pets.get(&user_address) {
        Some(found_pet) => pet = found_pet,
        None => pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME),
    }

    if pet.is_dead(time) {
        return Err(StdError::generic_err("Pet is dead :("));
    }
    if !pet.is_hungry(time) {
        return Err(StdError::generic_err("Pet is not hungry"));
    }
    if amount < Uint128(TOKENS_PER_FEEDING as u128) {
        return Err(StdError::generic_err(
            "You need more tokens to feed the pet",
        ));
    }
    pet.last_feed_time = time;
    pets.set(&user_address, &pet);
    let burn_msg = snip20::burn_msg(
        amount,
        None,
        BLOCK_SIZE,
        state.token_info.code_hash.clone(),
        state.token_info.address.clone(),
    )?;
    Ok(HandleResponse {
        messages: vec![burn_msg],
        log: vec![log("current_time", time), log("is_hungry", time)],
        data: None,
    })
}

fn try_set_name<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    name: String,
) -> StdResult<HandleResponse> {
    let sender = &env.message.sender;
    let cannonical_adr = deps.api.canonical_address(&sender)?;
    let mut pets = Pets::from_storage(&mut deps.storage);
    let time = env.block.time;
    let mut pet = match pets.get(&cannonical_adr) {
        Some(pet) => pet,
        None => Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME),
    };
    pet.name = name;
    pets.set(&cannonical_adr, &pet.clone());
    let serialized_response = to_binary(&HandleAnswer::SetName {
        status: ResponseStatus::Success
    })?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(serialized_response)
    })
}


fn try_get_name<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let sender = env.message.sender;
    let cannonical_adr = deps.api.canonical_address(&sender)?;
    let pets = Pets::from_storage(&mut deps.storage);
    let mut status = ResponseStatus::Success;
    let name = match pets.get(&cannonical_adr) {
        Some(pet) => pet.name.clone(),
        None => {
            status = ResponseStatus::Failure;
            "Not found".to_string()
        }
    };
    let serialized_response = to_binary(&HandleAnswer::GetName {
        name,
        status
    })?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(serialized_response)
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
