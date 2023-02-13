use std::str::FromStr;

use blockstack_lib::burnchains::Txid;
use blockstack_lib::types::chainstate::BurnchainHeaderHash;
use blockstack_lib::util::HexError;

use crate::stacks_node;

pub trait PegQueue {
    type Error: std::error::Error;

    fn sbtc_op(&self) -> Result<Option<SbtcOp>, Self::Error>;
    fn poll(&self) -> Result<(), Self::Error>;

    fn acknowledge(
        &self,
        txid: &Txid,
        burn_header_hash: &BurnchainHeaderHash,
    ) -> Result<(), Self::Error>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum SbtcOp {
    PegIn(stacks_node::PegInOp),
    PegOutRequest(stacks_node::PegOutRequestOp),
}

impl PegQueue for SqlitePegQueue {
    type Error = Error;

    fn sbtc_op(&self) -> Result<Option<SbtcOp>, Self::Error> {
        let maybe_entry = self.get_singe_entry_with_status(&Status::New)?;

        let Some(mut entry) = maybe_entry else {
            return Ok(None)
        };

        entry.status = Status::Pending;
        self.insert(&entry)?;

        Ok(Some(entry.op))
    }

    fn poll(&self) -> Result<(), Self::Error> {
        todo!();
    }

    fn acknowledge(
        &self,
        txid: &Txid,
        burn_header_hash: &BurnchainHeaderHash,
    ) -> Result<(), Self::Error> {
        let mut entry = self.get_entry(txid, burn_header_hash)?;

        entry.status = Status::Acknowledged;
        self.insert(&entry)?;

        Ok(())
    }
}

pub struct SqlitePegQueue {
    conn: rusqlite::Connection,
}

impl SqlitePegQueue {
    pub fn in_memory() -> Result<Self, Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        let self_ = Self { conn };
        self_.initialize()?;
        Ok(self_)
    }

    fn initialize(&self) -> Result<(), Error> {
        self.conn.execute(Self::sql_schema(), rusqlite::params![])?;

        Ok(())
    }

    fn insert(&self, entry: &Entry) -> Result<(), Error> {
        self.conn.execute(
            Self::sql_insert(),
            rusqlite::params![
                entry.txid.to_hex(),
                entry.burn_header_hash.to_hex(),
                serde_json::to_string(&entry.op)?,
                entry.status.as_str(),
            ],
        )?;

        Ok(())
    }

    fn get_singe_entry_with_status(&self, status: &Status) -> Result<Option<Entry>, Error> {
        Ok(self
            .conn
            .prepare(Self::sql_select_status())?
            .query_map(rusqlite::params![status.as_str()], Entry::from_row)?
            .next()
            .transpose()?)
    }

    fn get_entry(
        &self,
        txid: &Txid,
        burn_header_hash: &BurnchainHeaderHash,
    ) -> Result<Entry, Error> {
        Ok(self.conn.prepare(Self::sql_select_pk())?.query_row(
            rusqlite::params![txid.to_hex(), burn_header_hash.to_hex()],
            Entry::from_row,
        )?)
    }

    const fn sql_schema() -> &'static str {
        r#"
        CREATE TABLE sbtc_ops (
            txid TEXT NOT NULL,
            burn_header_hash TEXT NOT NULL,
            op TEXT NOT NULL,
            status TEXT NOT NULL,

            PRIMARY_KEY(txid, burn_header_hash),
        ) 
        "#
    }

    const fn sql_insert() -> &'static str {
        r#"
        REPLACE INTO sbtc_ops (txid, burn_header_hash, op, status) VALUES (?1, ?2, ?3, ?4)
        "#
    }

    const fn sql_select_status() -> &'static str {
        r#"
        SELECT (txid, burn_header_hash, op, status) FROM sbtc_ops WHERE status=?1
        "#
    }

    const fn sql_select_pk() -> &'static str {
        r#"
        SELECT (txid, burn_header_hash, op, status) FROM sbtc_ops WHERE txid=?1 AND burn_header_hash=?2
        "#
    }
}

struct Entry {
    burn_header_hash: BurnchainHeaderHash,
    txid: Txid,
    op: SbtcOp,
    status: Status,
}

impl Entry {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let burn_header_hash =
            BurnchainHeaderHash::from_hex(&row.get::<_, String>(0)?).map_err(Error::from)?;
        let txid = Txid::from_hex(&row.get::<_, String>(1)?).map_err(Error::from)?;
        let op: SbtcOp = serde_json::from_str(&row.get::<_, String>(2)?).map_err(Error::from)?;
        let status: Status = row.get::<_, String>(3)?.parse()?;

        Ok(Self {
            burn_header_hash,
            txid,
            op,
            status,
        })
    }
}

enum Status {
    New,
    Pending,
    Acknowledged,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Pending => "pending",
            Self::Acknowledged => "acknowledged",
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "new" => Self::New,
            "pending" => Self::Pending,
            "acknowledged" => Self::Acknowledged,
            other => return Err(Error::UnrecognizedStatusString(other.to_owned())),
        })
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
