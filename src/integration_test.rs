#![cfg(test)]

use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{coins, Addr, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor, BankKeeper};

use crate::msg::{ExecuteMsg, QueryMsg, AllBeveragesResponse};

fn mock_app() -> App {
    let env = mock_env();
    let api = MockApi::default();
    let bank = BankKeeper::new();

    App::new(api, env.block, bank, MockStorage::new())
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
    let owner = Addr::unchecked("owner");
    let init_funds = coins(2000, "luna");

    let mut router = mock_app();

    router.init_bank_balance(&owner, init_funds).unwrap();

    let contract_id = router.store_code(contract_vending_machine());
    let msg = crate::msg::InstantiateMsg {
        name: "COFFEE TOKEN".to_string(),
        symbol: "CT".to_string(),
        decimals: 2
    };

    let contract_addr = router
        .instantiate_contract(contract_id, owner.clone(), &msg, &[], "CT", None)
        .unwrap();
    
    // Add beverage test
    // NAME: AMERICANO
    // PRICE: 40 COFFEE-TOKENS
    // AMOUNT: 3
    let add_beverage_query = ExecuteMsg::AddBeverage {
        name: "AMERICANO".to_string(),
        amount: Uint128::from(3u64),
        price: Uint128::from(40u64)
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &add_beverage_query, &[])
        .unwrap();
    
    // Set price for AMERICANO
    // PRICE: 50 COFFEE-TOKENS
    let set_price_query = ExecuteMsg::SetPrice {
        name: "AMERICANO".to_string(),
        price: Uint128::from(50u64)
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &set_price_query, &[])
        .unwrap();

    // Fill Up AMERICANO
    // AMOUNT: 3+7=10
    let fill_up_beverage_query = ExecuteMsg::FillUpBeverage {
        name: "AMERICANO".to_string(),
        amount: Uint128::from(7u64)
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &fill_up_beverage_query, &[])
        .unwrap();

    // Give tokens to owner
    // Tokens: 60
    let give_tokens_query = ExecuteMsg::GiveTokens {
        address: owner.clone().into_string(),
        tokens: Uint128::from(60u64)
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &give_tokens_query, &[])
        .unwrap();

    // Buy AMERICANO
    // AMOUNT: 10 - 1 = 9
    let buy_beverage_query = ExecuteMsg::BuyBeverage {
        name: "AMERICANO".to_string()
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &buy_beverage_query, &[])
        .unwrap();

    // Withdraw supply
    let withdraw_query = ExecuteMsg::Withdraw {
        coin_amount: Uint128::from(30u64)
    };

    router
        .execute_contract(owner.clone(), contract_addr.clone(), &withdraw_query, &[])
        .unwrap();

    // Query
    let query: AllBeveragesResponse = router
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::BeverageList {offset: None, limit: None})
        .unwrap();

    for beverage in query.beverages {
        match beverage.name.as_str() {
            "AMERICANO" => {
                assert_eq!(beverage.price, Uint128::from(50u64));
                assert_eq!(beverage.amount, Uint128::from(9u64))
            }
            _ => panic!("There is no other coffee than AMERICANO"),
        }
    }
}
