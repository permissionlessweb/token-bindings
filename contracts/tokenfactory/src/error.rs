use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenFactoryError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid subdenom: {subdenom:?}")]
    InvalidSubdenom { subdenom: String },

    #[error("Invalid denom: {denom:?} {message:?}")]
    InvalidDenom { denom: String, message: String },

    #[error("denom does not exist: {denom:?}")]
    DenomDoesNotExist { denom: String },

    #[error("amount was zero, must be positive")]
    ZeroAmount {},
}

impl PartialEq for TokenFactoryError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Handle the `Std` variant by comparing the error message
            (TokenFactoryError::Std(e1), TokenFactoryError::Std(e2)) => {
                e1.to_string() == e2.to_string()
            }
            // For other variants, compare them directly
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}
