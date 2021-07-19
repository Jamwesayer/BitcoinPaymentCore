use crate::business::model::*;
use thiserror::Error;

// ErrorEntity
// pub struct ErrorEntity {
//     label: String,
//     error_level: u32,
//     code: u32
// }

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("[Low-1] The following item: {0}, was not found for {1}")]
    NotFoundInDatabase(String, String),
    #[error("[Low-2] The item '{0}', does already exist")]
    AlreadyExistInDatabase(String),
    #[error("[Low-2] The address could not be made for the current shop")]
    AddressCouldNotBeMadeOnBlockchain,
    // RefundFailed,
    // ErrorBlockchain
}
