#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128,
};

use cw2::set_contract_version;
use cw20_base::state::{MinterData, TokenInfo, BALANCES, TOKEN_INFO};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{AllBeveragesDetails, AllBeveragesResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BeverageInfo, VENDING_MACHINE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atticlab-task";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_AMOUNT: Uint128 = Uint128::new(50);
const DEFAULT_PAGE_SIZE: u32 = 10;
const MAX_PAGE_SIZE: u32 = 30;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply: Uint128::new(0),
        mint: Some(MinterData {
            minter: info.sender,
            cap: Option::from(Uint128::new(10000)),
        }),
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddBeverage {
            name,
            amount,
            price,
        } => Ok(add_beverage(
            deps,
            env,
            info,
            name,
            Uint128::from(amount),
            Uint128::from(price),
        )?),
        ExecuteMsg::SetPrice { name, price } => {
            Ok(set_price(deps, env, info, name, Uint128::from(price))?)
        }
        ExecuteMsg::FillUpBeverage { name, amount } => Ok(fill_up_beverage(
            deps,
            env,
            info,
            name,
            Uint128::from(amount),
        )?),
        ExecuteMsg::BuyBeverage { name } => Ok(buy_beverage(deps, env, info, name)?),
        ExecuteMsg::Withdraw { coin_amount } => {
            Ok(withdraw(deps, env, info, Uint128::from(coin_amount))?)
        }
        ExecuteMsg::GiveTokens { address, tokens } => Ok(give_tokens(
            deps,
            env,
            info,
            address,
            Uint128::from(tokens),
        )?),
    }
}

///
/// Add beverage to vendiing machine
///
pub fn add_beverage(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    amount: Uint128,
    price: Uint128,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if price == Uint128::zero() {
        return Err(ContractError::InvalidZeroPrice {});
    }

    if amount > MAX_AMOUNT {
        return Err(ContractError::AmountTooBig {});
    }

    if VENDING_MACHINE.has(deps.storage, &name) {
        return Err(ContractError::IsAlreadyInVendingMachine {});
    }

    VENDING_MACHINE.save(deps.storage, &name, &BeverageInfo { amount, price })?;

    let res = Response::new()
        .add_attribute("NAME", name)
        .add_attribute("AMOUNT", amount)
        .add_attribute("PRICE", price);

    return Ok(res);
}

///
/// Set price for beverage
///
pub fn set_price(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    price: Uint128,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if price == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    if !VENDING_MACHINE.has(deps.storage, &name) {
        return Err(ContractError::BeverageNotFound {});
    }

    VENDING_MACHINE.update(deps.storage, &name, |value| -> StdResult<_> {
        let mut value = value.unwrap_or_default();
        value.price = price;
        Ok(value)
    })?;

    let res = Response::new()
        .add_attribute("NAME", name)
        .add_attribute("PRICE", price);

    Ok(res)
}

///
/// Fill up the vending machine with beverage
///
pub fn fill_up_beverage(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    if !VENDING_MACHINE.has(deps.storage, &name) {
        return Err(ContractError::BeverageNotFound {});
    }

    if VENDING_MACHINE.load(deps.storage, &name)?.amount + amount > MAX_AMOUNT {
        return Err(ContractError::AmountTooBig {});
    }

    let beverage_info = VENDING_MACHINE.update(deps.storage, &name, |value| -> StdResult<_> {
        let mut value = value.unwrap_or_default();
        value.amount += amount;
        Ok(value)
    })?;

    let res = Response::new()
        .add_attribute("NAME", name)
        .add_attribute("AMOUNT", beverage_info.amount);

    Ok(res)
}

///
/// Buy beverage
///
pub fn buy_beverage(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    if !VENDING_MACHINE.has(deps.storage, &name) {
        return Err(ContractError::BeverageNotFound {});
    }

    let beverage = VENDING_MACHINE.load(deps.storage, &name)?;

    if beverage.amount.is_zero() {
        return Err(ContractError::BevaregeIsOver {});
    }

    let balance = BALANCES.load(deps.storage, &info.sender)?;

    if beverage.price > balance {
        return Err(ContractError::NotEnoughTokens {
            needed: String::from(beverage.price),
            given: String::from(balance),
        });
    }

    BALANCES.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default() - beverage.price)
        },
    )?;

    VENDING_MACHINE.update(deps.storage, &name, |beverage| -> StdResult<_> {
        let mut beverage = beverage.unwrap_or_default();

        beverage.amount -= Uint128::new(1);

        Ok(beverage)
    })?;

    TOKEN_INFO.update(deps.storage, |mut token| -> StdResult<_> {
        token.total_supply += beverage.amount;
        Ok(token)
    })?;

    let res = Response::new().add_attribute("PURCHASE", name);

    Ok(res)
}

///
/// Withdraw supplied tokens
///
pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    count_amount: Uint128,
) -> Result<Response, ContractError> {
    let config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    TOKEN_INFO.update(deps.storage, |mut token| -> StdResult<_> {
        token.total_supply -= count_amount;
        Ok(token)
    })?;

    let res = Response::new().add_attribute("WITHDRAWN", count_amount);
    Ok(res)
}

///
/// Gives tokens to the user
///
pub fn give_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
    tokens: Uint128,
) -> Result<Response, ContractError> {
    let mut config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.total_supply += tokens;
    if let Some(limit) = config.get_cap() {
        if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    let address = deps.api.addr_validate(&address)?;

    let balance = BALANCES.update(deps.storage, &address, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default() + tokens.clone())
    })?;

    let res = Response::new()
        .add_attribute("USER_ADDRESS", address)
        .add_attribute("BALANCE", balance);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BeverageList { offset, limit } => {
            to_binary(&query_beverage_list(deps, offset, limit)?)
        }
    }
}

///
/// Query all beverages in the vendor machine
///
pub fn query_beverage_list(
    deps: Deps,
    offset: Option<String>,
    limit: Option<u32>,
) -> StdResult<AllBeveragesResponse> {
    let limit = limit.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE) as usize;
    let offset = offset.map(Bound::exclusive);

    let beverages = VENDING_MACHINE
        .range(deps.storage, offset, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let item = item.unwrap_or_default();
            AllBeveragesDetails {
                name: std::str::from_utf8(&item.0).unwrap().to_string(),
                price: item.1.price,
                amount: item.1.amount,
            }
        })
        .collect();

    Ok(AllBeveragesResponse { beverages })
}
