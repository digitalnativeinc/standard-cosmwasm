use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub manager: Addr,
    pub factory: Addr,
    pub debt: Uint128,
    pub collateral: String,
    pub vault_id: u64,
    pub borrow: Uint128,
    pub last_updated: Timestamp,
    pub ex_sfr: u128
}

pub const STATE: Item<State> = Item::new("state");
