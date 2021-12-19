#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Uint128, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{
    AllBeveragesDetails, AllBeveragesResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg,
};
use crate::state::{BeverageInfo, ContractConfig, CONFIG, VENDING_MACHINE};
use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw_storage_plus::Bound;

// version info for migration info
const CONTRACT_NAME: &str = "atticlab-task";
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

    CONFIG.save(
        deps.storage,
        &ContractConfig {
            admin_addr: info.sender,
            balance: Cw20CoinVerified {
                address: deps.api.addr_validate(&msg.token_addr)?,
                amount: Uint128::zero(),
            },
        },
    )?;

    Ok(Response::new())
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
        ExecuteMsg::Receive(msg) => receive(deps, info, msg),
        ExecuteMsg::Withdraw { amount } => Ok(withdraw(deps, env, info, Uint128::from(amount))?),
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
    let config = CONFIG.load(deps.storage)?;
    if config.admin_addr.as_ref() != info.sender {
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
    let config = CONFIG.load(deps.storage)?;
    if config.admin_addr.as_ref() != info.sender {
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
    let config = CONFIG.load(deps.storage)?;
    if config.admin_addr.as_ref() != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
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
/// Receive CW20 Coffeee tokens
///
pub fn receive(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;
    let balance = Balance::Cw20(Cw20CoinVerified {
        address: info.sender,
        amount: wrapper.amount,
    });
    match msg {
        ReceiveMsg::BuyBeverage { name } => {
            buy_beverage(deps, name, balance)
        }
    }
}

///
/// Buy beverage
///
pub fn buy_beverage(
    deps: DepsMut,
    name: String,
    balance: Balance
) -> Result<Response, ContractError> {
    let beverage = VENDING_MACHINE.load(deps.storage, &name)?;

    if beverage.amount.is_zero() {
        return Err(ContractError::OutOfStock {});
    }

    let config = CONFIG.load(deps.storage)?;
    match balance {
        Balance::Native(_) => return Err(ContractError::WrongToken {}),
        Balance::Cw20(token) => {
            if token.address == config.balance.address {
                if token.amount < beverage.price {
                    return Err(ContractError::NotEnoughTokens {});
                }

                VENDING_MACHINE.update(deps.storage, &name, |beverage| -> StdResult<_> {
                    let mut beverage = beverage.unwrap_or_default();

                    beverage.amount -= Uint128::new(1);

                    Ok(beverage)
                })?;

                CONFIG.update(deps.storage, |mut conf| -> StdResult<_> {
                    conf.balance.amount = conf.balance.amount + beverage.price;

                    Ok(conf)
                })?;

                let res = Response::new().add_attribute("PURCHASED", &name);

                return Ok(res);
            }
            return Err(ContractError::WrongToken {});
        }
    };
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
    let config = CONFIG.load(deps.storage)?;
    if config.admin_addr.as_ref() != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg = Cw20ExecuteMsg::Transfer {
        recipient: config.admin_addr.to_string(),
        amount: count_amount,
    };

    let res = Response::new()
        .add_attribute("WITHDRAWN", count_amount)
        .add_message(WasmMsg::Execute {
            contract_addr: config.balance.address.to_string(),
            msg: to_binary(&msg)?,
            funds: vec![],
        });

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
