use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static PET_KEY: &[u8] = b"pet";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // time is seconds since epoch begin (Jan. 1, 1970)
    pub last_feed_time: u64,
    pub owner: CanonicalAddr,
}

pub fn pet<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, PET_KEY)
}

pub fn pet_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, PET_KEY)

}
// pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
//     singleton(storage, PET_KEY)
// }

// pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
//     singleton_read(storage, PET_KEY)
// }
