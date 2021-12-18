use cosmwasm_std::{Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw20::Expiration;

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

    /// Implements CW20. Transfer is a base message to move tokens to another account without triggering actions
    Transfer { recipient: String, amount: Uint128 },
    /// Implements CW20. Burn is a base message to destroy tokens forever
    Burn { amount: Uint128 },
    /// Implements CW20.  Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Implements CW20 "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Implements CW20 "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Implements CW20 "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Implements CW20 "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Implements CW20 "approval" extension. Destroys tokens forever
    BurnFrom { owner: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BeverageList {
        offset: Option<String>,
        limit: Option<u32>,
    },

    /// Implements CW20.
    Balance { address: String },
    TokenInfo {},
    Allowance { owner: String, spender: String },
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
