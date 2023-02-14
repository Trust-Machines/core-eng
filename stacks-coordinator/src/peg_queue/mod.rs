use blockstack_lib::burnchains::Txid;
use blockstack_lib::types::chainstate::BurnchainHeaderHash;
use blockstack_lib::util::HexError;

use crate::stacks_node;

mod sqlite_peg_queue;

pub use sqlite_peg_queue::SqlitePegQueue;

pub trait PegQueue {
    type Error: std::error::Error;

    fn sbtc_op(&self) -> Result<Option<SbtcOp>, Self::Error>;
    fn poll<N: stacks_node::StacksNode>(&self, stacks_node: &N) -> Result<(), Self::Error>;

    fn acknowledge(
        &self,
        txid: &Txid,
        burn_header_hash: &BurnchainHeaderHash,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SbtcOp {
    PegIn(stacks_node::PegInOp),
    PegOutRequest(stacks_node::PegOutRequestOp),
}

impl SbtcOp {
    pub fn as_peg_in(&self) -> Option<&stacks_node::PegInOp> {
        match self {
            Self::PegIn(op) => Some(op),
            _ => None,
        }
    }

    pub fn as_peg_out_request(&self) -> Option<&stacks_node::PegOutRequestOp> {
        match self {
            Self::PegOutRequest(op) => Some(op),
            _ => None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http network error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("JSON serialization failure: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Did not recognize status string: {0}")]
    UnrecognizedStatusString(String),

    #[error("Hex codec error: {0}")]
    HexError(#[from] HexError),

    #[error("Entry does not exist")]
    EntryDoesNotExist,
}

// Workaround to allow non-perfect conversions in `Entry::from_row`
impl From<Error> for rusqlite::Error {
    fn from(err: Error) -> Self {
        Self::InvalidColumnType(0, err.to_string(), rusqlite::types::Type::Text)
    }
}
