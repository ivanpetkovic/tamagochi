use secret_toolkit::snip20;

use cosmwasm_std::{
    from_binary, log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage, Uint128,
};

use crate::common::Minutes;
use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, ResponseStatus};
use crate::pet::{self, Pet};
use crate::state::{config_read, Pets, ReadonlyPets, State, TokenInfo, config};

const BLOCK_SIZE: usize = 256;
const DEFAULT_SATIATED_TIME: Minutes = 180;
const DEFAULT_STARVING_TIME: Minutes = 60;
const TOKENS_PER_FEEDING: u16 = 100;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let config_state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        token_info: TokenInfo {
            address: HumanAddr(msg.token_address.clone()),
            code_hash: msg.token_code_hash.clone(),
        },
    };

    config(&mut deps.storage).save(&config_state)?;

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
        HandleMsg::Create { name} => try_create_pet(deps, &env, name)
    }
}


fn try_create_pet<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    name: String
) -> StdResult<HandleResponse> {
    let time = env.block.time;
    let mut pets = Pets::from_storage(&mut deps.storage);
    let user_address = deps.api.canonical_address(&env.message.sender)?;
    let pet = Pet::new(time, DEFAULT_SATIATED_TIME, DEFAULT_STARVING_TIME, Some(&name));
    pets.set(&user_address, &pet);
    Ok(HandleResponse::default())
}

fn try_feed<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    sender: HumanAddr,
    amount: Uint128,
) -> StdResult<HandleResponse> {
    let time = env.block.time;
    let state = match config_read(&deps.storage).load() {
        Ok(state) => state,
        Err(err) => {
            return Err(StdError::not_found("Config not found"));
        }
    };
    let mut pets = Pets::from_storage(&mut deps.storage);
    let user_address = deps.api.canonical_address(&sender)?;
    let mut pet ;

    println!("pet set");
    if let Some(found_pet) = pets.get(&user_address) {
        pet = found_pet;
    } else {
        return Err(StdError::generic_err("You don't have a pet"));
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
        data: Some(to_binary(&HandleAnswer::Feed {
            status: ResponseStatus::Success,
        })?),
    })
}

fn try_set_name<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: &Env,
    name: String,
) -> StdResult<HandleResponse> {
    let sender = &env.message.sender;
    let canonical_adr = deps.api.canonical_address(sender)?;
    let mut pets = Pets::from_storage(&mut deps.storage);
    let mut pet = match pets.get(&canonical_adr) {
        Some(pet) => pet,
        None => return Err(StdError::not_found("Pet not found")),
    };
    pet.name = name;
    pets.set(&canonical_adr, &pet.clone());
    let serialized_response = to_binary(&HandleAnswer::SetName {
        status: ResponseStatus::Success,
    })?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(serialized_response),
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pet {
            address,
            viewing_key,
        } => query_pet(&deps, address, viewing_key),
    }
}

fn query_pet<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    sender: HumanAddr,
    viewing_key: String,
) -> StdResult<Binary> {
    let cannonical_adr = deps.api.canonical_address(&sender)?;
    let pets = ReadonlyPets::from_storage(&deps.storage);
    // TODO: auth user using the viewing_key
    let pet = if let Some(pet) = pets.get(&cannonical_adr) {
        pet
    } else {
        return Err(StdError::not_found(format!(
            "Pet not found for {}",
            &sender
        )));
    };

    let serialized_response = to_binary(&QueryAnswer::Pet { pet })?;
    Ok(serialized_response)
}

#[cfg(test)]
mod tests {
    use std::time;

    use crate::state::ReadonlyPets;

    use super::*;
    use bincode2::config;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{coins, CanonicalAddr};

    const FOOD_ADDRESS: &str = "secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg";
    const FOOD_CODE_HASH: &str = "E6687CD1C4E4ED16712CD7BD4CED08D7E01E7A95E6EA459773BF0C1851F2BA7F";
    const SENDERS: [&'static str; 3] = ["cartman", "kyle", "kenny"];

    // Helper functions

    fn init_helper(
        initial_pets: &Vec<Pet>,
    ) -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps: Extern<MockStorage, MockApi, MockQuerier> = mock_dependencies(20, &[]);
        let init_msg = InitMsg {
            token_address: FOOD_ADDRESS.to_string(),
            token_code_hash: FOOD_CODE_HASH.to_string(),
        };
        let mut pets = Pets::from_storage(&mut deps.storage);
        initial_pets.iter().enumerate().for_each(|(i, pet)| {
            let cannonical_address = deps
                .api
                .canonical_address(&HumanAddr(SENDERS[i].to_string()))
                .unwrap();
            // let serialized = bincode2::serialize(&pet).unwrap();
            // println!(
            //     "adr:{}, bin_adr:{:?} ser:{:?}, pet:{:?}",
            //     &cannonical_address,
            //     cannonical_address.as_slice(),
            //     &serialized,
            //     &pet
            // );
            pets.set(&cannonical_address, &pet);
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
        pets.push(Pet::new(
            time,
            DEFAULT_SATIATED_TIME,
            DEFAULT_STARVING_TIME,
            Some("Ethan"),
        ));
        pets.push(Pet::new(time, 1, 20, Some("Frey")));
        pets
    }
    
    /// Converts given address string into canonical address
    fn to_canonical<A: Api>(address: &str, api: &A) -> CanonicalAddr {
        api.canonical_address(&HumanAddr(address.to_string()))
            .unwrap()
    }

    #[test]
    fn proper_initialization() {
        let mocked_pets = create_pets();
        let (init_result, mut deps) = init_helper(&mocked_pets);

        let response = init_result.unwrap();
        assert_eq!(1, response.messages.len());

        {
            let pets = Pets::from_storage(&mut deps.storage);
            mocked_pets.iter().enumerate().for_each(|(i, mocked_pet)| {
                let canonical_address = to_canonical(SENDERS[i], &deps.api);
                let pet = pets.get(&canonical_address).unwrap();
                assert_eq!(pet.color, mocked_pet.color);
            })
        }

        { // ReadonlyPets and Pets should access the same storage
            let pets = ReadonlyPets::from_storage(&deps.storage);
            mocked_pets.iter().enumerate().for_each(|(i, mocked_pet)| {
                let canonical_address = to_canonical(SENDERS[i], &deps.api);
                let pet = pets.get(&canonical_address).unwrap();
                assert_eq!(pet.color, mocked_pet.color);
            })
        }
    }

    #[test]
    fn test_query_pet() {
        let mocked_pets = create_pets();
        let (_init_result, deps) = init_helper(&mocked_pets);
        let pets = ReadonlyPets::from_storage(&deps.storage);

        {
            let msg = QueryMsg::Pet {
                address: HumanAddr(SENDERS[0].to_string()),
                viewing_key: "key".to_string(),
            };
            let answer_res: StdResult<QueryAnswer> = from_binary(&query(&deps, msg).unwrap());
            let answer = answer_res.unwrap();
            match answer {
                QueryAnswer::Pet { pet } => {
                    assert_eq!(pet.name, mocked_pets[0].name);
                }
            }
        }

        {
            // sender who didn't create a pet should not be able to get its name
            let msg = QueryMsg::Pet {
                address: HumanAddr("unknown sender".to_string()),
                viewing_key: "key2".to_string(),
            };
            let query_res = query(&deps, msg);
            assert_eq!(query_res.is_err(), true);
        }

        // sender who didn't create a pet should not be able to get its name (direct storage access version)
        let canonical_address = to_canonical("some sender", &deps.api);
        let pet_res = pets.get(&canonical_address);
        assert_eq!(pet_res.is_none(), true);
    }

    #[test]
    fn test_set_name() {
        let mocked_pets = create_pets();
        let (_init_result, mut deps) = init_helper(&mocked_pets);
        let creator = SENDERS[0];
        let new_name = "Stan";
        let viewing_key = "proper-viewing-key".to_string();
        let env = mock_env(creator, &coins(1000, "FOOD"));
        let msg = HandleMsg::SetName {
            name: new_name.to_string(),
        };
        let serialized_answer = handle(&mut deps, env, msg).unwrap().data.unwrap();
        match from_binary(&serialized_answer).unwrap() {
            HandleAnswer::SetName { status } => {
                assert_eq!(status, ResponseStatus::Success);
            }
            HandleAnswer::Feed { status } => assert!(false),
        }
        let query_res = query(
            &deps,
            QueryMsg::Pet {
                address: HumanAddr(creator.to_string()),
                viewing_key,
            },
        )
        .unwrap();
        match from_binary(&query_res).unwrap() {
            QueryAnswer::Pet { pet } => {
                assert_eq!(pet.name, new_name);
            }
        }
    }


    #[test]
    fn test_feed_pet() {
        let mocked_pets = create_pets();
        let (_init_result, mut deps) = init_helper(&mocked_pets);
        let creator = SENDERS[0];
        let food_token_address = HumanAddr(FOOD_ADDRESS.to_string());
        let viewing_key = "proper-viewing-key".to_string();
        let env = mock_env(creator, &coins(1000, "FOOD"));
        let owner = HumanAddr(SENDERS[0].to_string());
        let msg = HandleMsg::Receive {
            sender: owner.clone(),
            from: owner.clone(),
            amount: Uint128(100),
            msg: None,
        };
        let cfg = config_read(&deps.storage).load();
        println!("cfg {:?}", cfg);
        let res = handle(&mut deps, env, msg);
        println!("{:?}", res);
        let serialized_answer = res.unwrap().data.unwrap();
        match from_binary(&serialized_answer).unwrap() {
            HandleAnswer::Feed { status } => {
                assert_eq!(status, ResponseStatus::Success);
            }
            HandleAnswer::SetName { status } => {
                assert!(false);
            }
        }
        
    }

}
