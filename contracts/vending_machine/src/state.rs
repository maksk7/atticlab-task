use cw20::Cw20CoinVerified;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeverageInfo {
    pub amount: Uint128,
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractConfig {
    pub admin_addr: Addr,
    pub balance: Cw20CoinVerified,
}

pub const VENDING_MACHINE: Map<&str, BeverageInfo> = Map::new("vending_machine");
pub const CONFIG: Item<ContractConfig> = Item::new("config");
