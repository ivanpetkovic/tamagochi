use std::ops::Add;
use std::time::Duration;

use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{pet, pet_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        last_feed_time: env.block.time,
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    pet(&mut deps.storage).save(&state)?;

    debug_print!("Pet was born and fed, thanks to {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Feed {amount} => {
            let time = env.block.time;
            try_feed(amount, &mut deps.storage, &env)
        }
    }
}
type Seconds = Duration;

pub fn try_feed<S: Storage>(amount: u32, storage: &mut S, env: &Env) -> StdResult<HandleResponse> {
    let time =  env.block.time;
    
    pet(storage).update(|state| {
        let duration: Seconds = Duration::new(time - state.last_feed_time, 0);
        
        println!("Hours since fed {}", duration.as_secs() / 360);
        Ok(state)
    });
    Ok(HandleResponse::default())
}


// pub fn try_increment<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     _env: Env,
// ) -> StdResult<HandleResponse> {
//     pet(&mut deps.storage).update(|mut state| {
//         state.count += 1;
//         debug_print!("count = {}", state.count);
//         Ok(state)
//     })?;

//     debug_print("count incremented successfully");
//     Ok(HandleResponse::default())
// }


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

fn can_eat<S:Storage, A:Api, Q:Querier>(deps: &Extern<S, A, Q>) -> StdResult<bool> {
    Ok(true)
}

fn is_hungry<S:Storage, A:Api, Q:Querier>(deps: &Extern<S, A, Q>) -> StdResult<bool> {
    let state = pet_read(&deps.storage).load()?;
    // deps.
    // state.last_feed_time
    Ok(true)
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
