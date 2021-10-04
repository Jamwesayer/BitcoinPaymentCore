use thiserror::Error;
use diesel::result::Error;

#[derive(Debug, Error)]
pub enum DataSourceError {
    #[error("[Low-1] The following item: {0}, was not found for {1}")]
    NotFoundInDatabaseError(String, String),
    #[error("[Low-2] The item '{0}', does already exist")]
    AlreadyExistInDatabaseError(String),
    #[error("[Low-2] The address could not be made for the current shop")]
    AddressCouldNotBeMadeOnBlockchainError,
    #[error("[Low-2] Found the following error: {0}")]
    KnownError(Error)
}
