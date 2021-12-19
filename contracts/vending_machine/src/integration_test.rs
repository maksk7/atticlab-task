#![cfg(test)]

use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{coins, to_binary, Addr, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg};
use cw_multi_test::{App, BankKeeper, Contract, ContractWrapper, Executor};

use crate::msg::{AllBeveragesResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};

fn mock_app() -> App {
    let env = mock_env();
    let api = MockApi::default();
    let bank = BankKeeper::new();

    App::new(api, env.block, bank, MockStorage::new())
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

pub fn contract_vending_machine() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

#[test]
fn vending_machine_test() {
    let mut router = mock_app();

    let owner = Addr::unchecked("owner");
    let init_funds = coins(2000, "luna");

    router.init_bank_balance(&owner, init_funds).unwrap();

    let token_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: "Coffee Token".to_string(),
        symbol: "COFF".to_string(),
        decimals: 3,
        initial_balances: vec![Cw20Coin {
            address: owner.to_string(),
            amount: Uint128::new(3000),
        }],
        mint: None,
        marketing: None,
    };

    let token_addr = router
        .instantiate_contract(token_id, owner.clone(), &msg, &[], "CASH", None)
        .unwrap();

    let vending_machine_id = router.store_code(contract_vending_machine());
    let vending_machine_addr = router
        .instantiate_contract(
            vending_machine_id,
            owner.clone(),
            &InstantiateMsg {
                token_addr: token_addr.to_string(),
            },
            &[],
            "Vending machine",
            None,
        )
        .unwrap();

    let cw20_token = Cw20Contract(token_addr.clone());

    let owner_balance = cw20_token.balance(&router, owner.clone()).unwrap();
    let vending_machine_balance = cw20_token
        .balance(&router, vending_machine_addr.clone())
        .unwrap();

    assert_eq!(owner_balance, Uint128::new(3000));
    assert_eq!(vending_machine_balance, Uint128::zero());

    // Add beverage test
    // NAME: Americano
    // PRICE: 2 COFFEE-TOKENS
    // AMOUNT: 3
    let add_beverage_query = ExecuteMsg::AddBeverage {
        name: "Americano".to_string(),
        amount: Uint128::new(3),
        price: Uint128::new(2),
    };

    router
        .execute_contract(
            owner.clone(),
            vending_machine_addr.clone(),
            &add_beverage_query,
            &[],
        )
        .unwrap();

    // Set price for Americano
    // PRICE: 3 COFFEE-TOKENS
    let set_price_query = ExecuteMsg::SetPrice {
        name: "Americano".to_string(),
        price: Uint128::new(3),
    };

    router
        .execute_contract(
            owner.clone(),
            vending_machine_addr.clone(),
            &set_price_query,
            &[],
        )
        .unwrap();

    // Fill Up Americano
    // AMOUNT: 3+7=10
    let fill_up_beverage_query = ExecuteMsg::FillUpBeverage {
        name: "Americano".to_string(),
        amount: Uint128::new(7),
    };

    router
        .execute_contract(
            owner.clone(),
            vending_machine_addr.clone(),
            &fill_up_beverage_query,
            &[],
        )
        .unwrap();

    // Buy Americano
    let buy_americano_msg = ReceiveMsg::BuyBeverage {
        name: "Americano".to_string(),
    };

    let send_msg = Cw20ExecuteMsg::Send {
        contract: vending_machine_addr.to_string(),
        amount: Uint128::new(3),
        msg: to_binary(&buy_americano_msg).unwrap(),
    };

    router
        .execute_contract(owner.clone(), token_addr, &send_msg, &[])
        .unwrap();

    let owner_balance = cw20_token.balance(&router, owner.clone()).unwrap();
    let vending_machine_balance = cw20_token
        .balance(&router, vending_machine_addr.clone())
        .unwrap();

    assert_eq!(owner_balance, Uint128::new(2997));
    assert_eq!(vending_machine_balance, Uint128::new(3));

    // Withdraw 2 Coffee tokens
    let withdraw_msg = ExecuteMsg::Withdraw {
        amount: Uint128::new(2),
    };

    router
        .execute_contract(
            owner.clone(),
            vending_machine_addr.clone(),
            &withdraw_msg,
            &[],
        )
        .unwrap();

    let owner_balance = cw20_token.balance(&router, owner.clone()).unwrap();
    let vending_machine_balance = cw20_token
        .balance(&router, vending_machine_addr.clone())
        .unwrap();

    assert_eq!(owner_balance, Uint128::new(2999));
    assert_eq!(vending_machine_balance, Uint128::new(1));

    let query: AllBeveragesResponse = router
        .wrap()
        .query_wasm_smart(
            &vending_machine_addr,
            &QueryMsg::BeverageList {
                limit: None,
                offset: None,
            },
        )
        .unwrap();

    for beverage in query.beverages {
        match beverage.name.as_str() {
            "Americano" => {
                assert_eq!(beverage.price, Uint128::new(3));
                assert_eq!(beverage.amount, Uint128::new(9))
            }
            _ => (),
        }
    }
}
