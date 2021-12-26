use cosmwasm_std::testing::{MockStorage, MockApi, MockQuerier, mock_dependencies};
use secret_toolkit::snip20;
use std::time::{self, Duration};

use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};

use crate::msg::{HandleAnswer, HandleMsg, InitMsg, Minutes, QueryMsg, ResponseStatus};
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
    // let mut pets = Pets::from_storage(&mut deps.storage);
    let user_address = deps.api.canonical_address(&sender)?;
    let mut pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, None);
  
    // match pets.get(&user_address) {
    //     Some(found_pet) => pet = found_pet,
    //     None => pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME),
    // }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("pet", format!("{:?}", &pet))],
        data: None,
    })
}

fn try_feed2<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let time = env.block.time;
    let state = config_read(&deps.storage).load()?;
    let mut pets = Pets::from_storage(&mut deps.storage);
    let user_address = deps.api.canonical_address(&sender)?;
    let mut pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, None);
  
    match pets.get(&user_address) {
        Some(found_pet) => pet = found_pet,
        None => pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, None),
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
    // let mut pet = match pets.get(&cannonical_adr) {
    //     Some(pet) => pet,
    //     None => Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME),
    // };
    let mut pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, None);
    pet.name = name;
    pets.set(&cannonical_adr, &pet.clone());
    let serialized_response = to_binary(&HandleAnswer::SetName {
        status: ResponseStatus::Success,
    })?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(serialized_response),
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

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("name", name), log("eadi", "Radi")],
        data: None
    })

    // let serialized_response = to_binary(&HandleAnswer::GetName {
    //     name,
    //     status
    // })?;
    // Ok(HandleResponse {
    //     messages: vec![],
    //     log: vec![],
    //     data: Some(serialized_response)
    // })
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
    use crate::state::ReadonlyPets;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError, CanonicalAddr};

    const FOOD_ADDRESS: &str = "secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg";
    const FOOD_CODE_HASH: &str = "E6687CD1C4E4ED16712CD7BD4CED08D7E01E7A95E6EA459773BF0C1851F2BA7F";
    const senders: [&'static str; 3] = ["cartman", "kyle", "kenny"];
    

   // Helper functions

   fn init_helper(initial_pets: &Vec<Pet>) -> (
    StdResult<InitResponse>,
    Extern<MockStorage, MockApi, MockQuerier>,
) {
    let mut deps: Extern<MockStorage, MockApi, MockQuerier> = mock_dependencies(20, &[]);
    let init_msg = InitMsg { 
        token_address: FOOD_ADDRESS.to_string(),
        token_code_hash: FOOD_CODE_HASH.to_string()
     };
     // should I bother adding 20 bytes long address? snip20 implementation doesn't 
    initial_pets.iter().enumerate().for_each(|(i, pet)| {
        let cannonical_address = deps.api.canonical_address(&HumanAddr(senders[i].to_string())).unwrap();
        let serialized = bincode2::serialize(&pet).unwrap();
        println!("adr:{}, bin_adr:{:?} ser:{:?}, pet:{:?}", &cannonical_address, cannonical_address.as_slice(), &serialized, &pet);
        deps.storage.set(cannonical_address.as_slice(), &serialized);
    });
    let env = mock_env("creator", &coins(1000, "FOOD"));
    (init(&mut deps, env, init_msg), deps)
}

fn create_pets() -> Vec<Pet> {
    let mut pets: Vec<Pet> = Vec::new();
    let time = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .expect("Back to the future")
        .as_secs();
    pets.push(Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, Some("Ethan")));
    pets.push(Pet::new(time, 1, 20, Some("Frey")));
    pets

}
fn to_canonical<A:Api>(address: &str, api: &A) -> CanonicalAddr {
    api.canonical_address(&HumanAddr(address.to_string())).unwrap()
}
    #[test]
    fn proper_initialization() {
        let mocked_pets = create_pets();
        let (init_result, deps) = init_helper(&mocked_pets);
        // we can just call .unwrap() to assert this was a success
        let response = init_result.unwrap();
        // let callback = snip20::register_receive_msg(
        //     pet_contract_hash.clone(),
        //     None,
        //     BLOCK_SIZE,
        //     msg.token_code_hash.clone(),
        //     HumanAddr(msg.token_address.clone()),
        // )?;
        assert_eq!(1, response.messages.len());
        // println!("{:}", &deps.storage);
        let pets = ReadonlyPets::from_storage(&deps.storage);
        let canonical_address = to_canonical(senders[0], &deps.api);
        let pet = pets.get(&canonical_address).unwrap();
        println!("color {}", pet.color);
        assert_eq!(pet.color, "red");//mocked_pets[0].color);
        // assert_eq!(FOOD_ADDRESS, deps.storage.)
        // it worked, let's query the state
        // let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    
    //#[test]
    fn test_init_sanity() {
        // let (init_result, deps) = init_helper(create_pets());

        // let config = ReadonlyConfig::from_storage(&deps.storage);
        // let constants = config.constants().unwrap();
        // assert_eq!(config.total_supply(), 5000);
        // assert_eq!(config.contract_status(), ContractStatusLevel::NormalRun);
        // assert_eq!(constants.name, "sec-sec".to_string());
        // assert_eq!(constants.admin, HumanAddr("admin".to_string()));
        // assert_eq!(constants.symbol, "SECSEC".to_string());
        // assert_eq!(constants.decimals, 8);
        // assert_eq!(
        //     constants.prng_seed,
        //     sha_256("lolz fun yay".to_owned().as_bytes())
        // );
        // assert_eq!(constants.total_supply_is_public, false);
    }

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
