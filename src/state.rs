use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeverageInfo {
    pub amount: Uint128,
    pub price: Uint128,
}

pub const VENDING_MACHINE: Map<&str, BeverageInfo> = Map::new("vending_machine");
