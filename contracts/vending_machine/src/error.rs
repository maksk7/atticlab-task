use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Invalid zero price")]
    InvalidZeroPrice {},

    #[error("Beverage is already in vending machine")]
    IsAlreadyInVendingMachine {},

    #[error("Beverage Amount cannot be more than 50")]
    AmountTooBig {},

    #[error("Beverage Not Found")]
    BeverageNotFound {},

    #[error("Out of stock")]
    OutOfStock {},

    #[error("Not enough tokens")]
    NotEnoughTokens {},

    #[error("Wrong token")]
    WrongToken {},
}
