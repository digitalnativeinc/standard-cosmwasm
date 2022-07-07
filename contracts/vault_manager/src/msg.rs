use std::ops::Add;

use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Initialize {
        vault_code_id_: u64,
        v1_: Addr,
        stablecoin_: Addr,
        admin_: Addr,
    },
    CreateVault {
        dAmount: Uint128,
    },
    SetVaultConfig {
        clt: String,
        c_decimal_: u64,
        pool_id_: u64,
        // Each rate is Percent with 5 decimals, e.g. 100% = 10000000
        mcr_: u64,
        lfr_: u64,
        sfr_: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    VaultConfigResponse { clt: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultConfigResponse {
    pub c_decimal: u64,
    pub pool_id: u64,
    pub mcr: u64,
    pub lfr: u64,
    pub sfr: u64,
}
