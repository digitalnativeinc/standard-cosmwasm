use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
    pub manager: Addr,
    pub collateral: String,
    pub debt: Addr,
    pub v1: Addr,
    pub amount: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
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
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    Factory {},
    Manager {},
    Debt {},
    V1 {},
    Collateral {},
    VaultId {},
    Borrow {},
    LastUpdated {},
    CreatedAt {},
    OutstandingPayment {}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositCollateralResponse {
    pub count: i32,
}


// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawCollateralResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowMoreResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PaybackResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloseVaultResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidateResponse {
    pub count: i32,
}

