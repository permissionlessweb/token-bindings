use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReflectError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Permission denied: the sender is not the current owner")]
    NotCurrentOwner { expected: String, actual: String },

    #[error("Messages empty. Must reflect at least one message")]
    MessagesEmpty,

    #[error("TODO: implement")]
    NotYetImplemented,
}

use std::cmp::PartialEq;

impl PartialEq for ReflectError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Handle the `Std` variant by comparing the error message
            (ReflectError::Std(e1), ReflectError::Std(e2)) => e1.to_string() == e2.to_string(),
            // For other variants, compare them directly
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}
