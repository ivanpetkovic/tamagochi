use std::fmt;

use bincode2::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, PrefixedStorage, ReadonlySingleton, Singleton};

use crate::pet::Pet;

pub static PET_KEY: &[u8] = b"pet";
pub static CONFIG_KEY: &[u8] = b"config";
pub const PREFIX_PETS: &[u8] = b"pets";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub code_hash: String,
    pub address: HumanAddr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub token_info: TokenInfo,
}


pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, PET_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub struct Pets<'a, S: Storage> {
    storage: PrefixedStorage<'a, S>,
}

// impl fmt::Display for Pets {
//     fn fmt(self: &Pets, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
//         self.storage.
//     }
// }

/// This was the only way to prevent code duplication of these methods because of the way
/// that `ReadonlyPrefixedStorage` and `PrefixedStorage` are implemented in `cosmwasm-std`
#[derive(Debug)]
pub struct ReadonlyPets<'a, S: Storage>(&'a S);

impl<'a, S: Storage> ReadonlyPets<'a, S> {
    pub fn get(&self, account: &CanonicalAddr) -> Option<Pet> {
        let account_bytes = account.as_slice();
        match self.0.get(account_bytes) {
            Some(res) => {
                let pet_result: Result<Pet, Error> = bincode2::deserialize(&res[..]);
                Some(pet_result.unwrap())
            },
            None => {
                println!("/n Not found pet for {}", &account);
                return None;
            }
        }
    }

    pub fn from_storage(storage: &'a S) -> Self {
        ReadonlyPets(storage)
    }
}

impl<'a, S: Storage> Pets<'a, S> {
    pub fn from_storage(storage: &'a mut S) -> Self {
        Self {
            storage: PrefixedStorage::new(PREFIX_PETS, storage),
        }
    }

    // pub fn from_storage_as_readonly(storage: &'a S) -> Self {
    //     Self {
    //         storage: ReadonlyStorage::new(PREFIX_PETS, storage),
    //     }
    // }

    fn as_readonly(&self) -> ReadonlyPets<PrefixedStorage<S>> {
        ReadonlyPets(&self.storage)
    }

    pub fn get(&self, account: &CanonicalAddr) -> Option<Pet> {
        self.as_readonly().get(account)
    }

    pub fn set(&mut self, account: &CanonicalAddr, pet: &Pet) {
        let serialized = bincode2::serialize(pet).unwrap();
        self.storage.set(account.as_slice(), &serialized)
    }
}
