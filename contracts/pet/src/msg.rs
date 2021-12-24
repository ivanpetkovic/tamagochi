use cosmwasm_std::{HumanAddr, Uint128, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


pub type Hours = u32;
pub type Minutes = u32;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub token_code_hash: String,
    pub token_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Receive {
        sender: HumanAddr,
        from: HumanAddr, 
        amount: Uint128,
        msg: Option<Binary>,
    },
    GetName {}, // hand to put in handle and not in query in order to find sender
    SetName {
        name: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    GetName {
        name: String,
        status: ResponseStatus
    },
    SetName {
        status: ResponseStatus
    }
}