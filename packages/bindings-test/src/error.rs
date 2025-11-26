use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Invalid full denom '{full_denom}'")]
    InvalidFullDenom { full_denom: String },

    #[error("Not admin of token, cannot perfrom action")]
    NotTokenAdmin,

    #[error("Token denom already exists, cannot create again")]
    TokenExists,

    #[error("Token denom was never created")]
    TokenDoesntExist,
}
