use cosmwasm_std::{HumanAddr, Uint128, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


pub type Hours = u32;
pub type Minutes = u32;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub token_code_hash: String,
    pub token_address: String,
    pub satiated_interval: Option<u32>,
    pub starving_interval: Option<u32>,
    pub pet_name: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Receive {
        sender: HumanAddr,
        from: HumanAddr, 
        amount: Uint128,
        msg: Option<Binary>,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    IsHungry,
    CanEat,
}

