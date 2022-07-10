use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub vault_id: u64,
    pub manager: String,
    pub collateral: String,
    pub debt: String,
    pub v1: String,
    pub borrow: Uint128,
    pub created_at: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // vault custom methods
    Liquidate {},
    WithdrawCollateral { amount: Uint128 },
    DepositCollateral { amount: Uint128},
    BorrowMore { amount: Uint128 },
    Paydebt { amount: Uint128 },
    CloseVault { amount: Uint128}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub vault_id: u64,
    pub manager: String,
    pub collateral: String,
    pub debt: String,
    pub v1: String,
    pub borrow: Uint128,
    pub created_at: u64,
    pub last_updated: u64,
    pub sfr: u64
}

