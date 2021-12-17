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

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Beverage is already in vending machine")]
    IsAlreadyInVendingMachine {},

    #[error("Beverage Amount cannot be more than 50")]
    AmountTooBig {},

    #[error("Beverage Not Found")]
    BeverageNotFound {},

    #[error("The beverage is over")]
    BevaregeIsOver {},

    #[error("Not enough COFFEETOKENS(needed {needed}, given {given})")]
    NotEnoughTokens {needed: String, given: String},
}
