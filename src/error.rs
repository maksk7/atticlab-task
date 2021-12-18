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
    NotEnoughTokens { needed: String, given: String },

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(err: cw20_base::ContractError) -> Self {
        match err {
            cw20_base::ContractError::Std(error) => ContractError::Std(error),
            cw20_base::ContractError::Unauthorized {} => ContractError::Unauthorized {},
            cw20_base::ContractError::CannotSetOwnAccount {} => {
                ContractError::CannotSetOwnAccount {}
            }
            cw20_base::ContractError::InvalidZeroAmount {} => ContractError::InvalidZeroAmount {},
            cw20_base::ContractError::Expired {} => ContractError::Expired {},
            cw20_base::ContractError::NoAllowance {} => ContractError::NoAllowance {},
            cw20_base::ContractError::CannotExceedCap {} => ContractError::CannotExceedCap {},
            cw20_base::ContractError::LogoTooBig {}
            | cw20_base::ContractError::InvalidPngHeader {}
            | cw20_base::ContractError::InvalidXmlPreamble {} => {
                ContractError::Std(StdError::generic_err(err.to_string()))
            }
        }
    }
}