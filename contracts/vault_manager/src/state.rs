use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct VaultConfig {
    /// Collateral Decimal
    pub c_decimal: u64,
    /// Maximum Collateral Ratio
    pub mcr: u64,
    /// Liquidation Fee Ratio
    pub lfr: u64,
    /// Stability Fee Ratio (interest rate)
    pub sfr: u64,
    /// Pool Id to get price
    pub pool_id: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Config {
    pub v1: String,
    pub stablecoin: String,
    pub factory: String,
    pub admin: String,
    pub vault_code_id: u64,
    pub initialized: bool
}

pub const VAULTCONFIG: Map<String, VaultConfig> = Map::new("vault_config");
pub const CONFIG: Item<Config> = Item::new("config");