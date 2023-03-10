use frost_coordinator::coordinator::Error as FrostCoordinatorError;
use frost_signer::net::HttpNetError;

use crate::peg_queue::Error as PegQueueError;

/// Helper that uses this module's error type
pub type Result<T> = std::result::Result<T, Error>;

/// Kinds of common errors used by stacks coordinator
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error occurred with the HTTP Relay
    #[error("Http Network Error: {0}")]
    HttpNetError(#[from] HttpNetError),
    /// Error occurred with the sBTC Contract
    #[error("sBTC Contract Error")]
    ContractError,
    /// Error occurred with the Frost Coordinator
    #[error("Frost Coordinator encountered an error: {0}")]
    FrostCoordinatorError(#[from] FrostCoordinatorError),
    /// Error occurred with peg queue
    #[error("Error occurred in the Peg Queue: {0}")]
    PegQueueError(#[from] PegQueueError),
    /// Error occurred reading a file
    #[error("Failed to read file: {0}")]
    FileReadingError(#[from] std::io::Error),
    /// Config parse error
    #[error("Failed to parse config file: {0}")]
    ConfigError(#[from] toml::de::Error),
}
