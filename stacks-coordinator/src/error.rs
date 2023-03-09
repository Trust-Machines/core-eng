use frost_coordinator::coordinator::Error as FrostCoordinatorError;
use frost_signer::net::HttpNetError;

use crate::peg_queue::Error as SqliteError;

/// Helper that uses this module's error type
pub type Result<T> = std::result::Result<T, Error>;

/// Kinds of common errors used by the parsers
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Feature is not yet implemented.
    #[error("Unimplemented Error")]
    Unimplemented,
    /// Error occurred with the HTTP Relay
    #[error("Http Network Error: {0}")]
    HttpNetError(#[from] HttpNetError),
    /// Error occurred with the sBTC Contract
    #[error("sBTC Contract Error")]
    ContractError,
    /// Error occurred with the Frost Coordinator
    #[error("Frost Coordinator Error: {0}")]
    FrostCoordinatorError(#[from] FrostCoordinatorError),
    #[error("Sqlite Error : {0}")]
    SqliteError(#[from] SqliteError),
}
