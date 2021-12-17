use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddBeverage {
        name: String,
        amount: Uint128,
        price: Uint128,
    },
    SetPrice {
        name: String,
        price: Uint128,
    },
    FillUpBeverage {
        name: String,
        amount: Uint128,
    },
    BuyBeverage {
        name: String,
    },
    Withdraw {
        coin_amount: Uint128,
    },
    GiveTokens {
        address: String,
        tokens: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BeverageList {
        offset: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllBeveragesDetails {
    pub name: String,
    pub amount: Uint128,
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AllBeveragesResponse {
    pub beverages: Vec<AllBeveragesDetails>,
}
