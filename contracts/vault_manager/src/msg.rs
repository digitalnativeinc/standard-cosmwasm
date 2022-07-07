use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    Initialize { vault_code_id: u64 },
    CreateVault { dAmount: Uint128 },
    InitializeConfig {
        clt: String,
        c_decimal_: u64,
        pool_id_ : u64,
        // Each rate is Percent with 5 decimals, e.g. 100% = 10000000
        mcr_: u64, 
        lfr_: u64,
        sfr_: u64,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitializeConfigResponse {
    pub pool_id: u64,
    pub clt: u64,
    pub mcr: u64,
    pub lfr: u64,
    pub sfr: u64,
}

