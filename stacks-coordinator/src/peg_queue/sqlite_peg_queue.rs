use std::path::Path;
use std::str::FromStr;

use rusqlite::Connection as RusqliteConnection;
use rusqlite::Error as SqliteError;
use rusqlite::Row as SqliteRow;

use blockstack_lib::burnchains::Txid;
use blockstack_lib::types::chainstate::BurnchainHeaderHash;
use blockstack_lib::util::HexError;

use crate::peg_queue::PegQueue;
use crate::peg_queue::SbtcOp;
use crate::stacks_node;

pub struct SqlitePegQueue {
    conn: rusqlite::Connection,
    start_block_height: u64,
}

impl SqlitePegQueue {
    pub fn new<P: AsRef<Path>>(path: P, start_block_height: u64) -> Result<Self, Error> {
        Self::from_connection(RusqliteConnection::open(path)?, start_block_height)
    }

    pub fn in_memory(start_block_height: u64) -> Result<Self, Error> {
        Self::from_connection(RusqliteConnection::open_in_memory()?, start_block_height)
    }

    fn from_connection(conn: RusqliteConnection, start_block_height: u64) -> Result<Self, Error> {
        let this = Self {
            conn,
            start_block_height,
        };
        this.conn.execute(Self::sql_schema(), rusqlite::params![])?;
        Ok(this)
    }

    fn insert(&self, entry: &Entry) -> Result<(), Error> {
        self.conn.execute(
            Self::sql_insert(),
            rusqlite::params![
                entry.txid.to_hex(),
                entry.burn_header_hash.to_hex(),
                entry.block_height as i64, // Stacks will crash before the coordinator if this is invalid
                serde_json::to_string(&entry.op)?,
                entry.status.as_str(),
            ],
        )?;

        Ok(())
    }

    fn get_single_entry_with_status(&self, status: &Status) -> Result<Option<Entry>, Error> {
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

    fn max_observed_block_height(&self) -> Result<u64, Error> {
        Ok(self
            .conn
            .query_row(
                Self::sql_select_max_burn_height(),
                rusqlite::params![],
                |row| {
                    Ok(row
                        .get::<_, i64>(0)
                        .unwrap_or(self.start_block_height as i64))
                },
            )
            .map(|count| count as u64)?)
    }

    const fn sql_schema() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS sbtc_ops (
            txid TEXT NOT NULL,
            burn_header_hash TEXT NOT NULL,
            block_height INTEGER NOT NULL,
            op TEXT NOT NULL,
            status TEXT NOT NULL,

            PRIMARY KEY(txid, burn_header_hash)
        )
        "#
    }

    const fn sql_insert() -> &'static str {
        r#"
        REPLACE INTO sbtc_ops (txid, burn_header_hash, block_height, op, status) VALUES (?1, ?2, ?3, ?4, ?5)
        "#
    }

    const fn sql_select_status() -> &'static str {
        r#"
        SELECT txid, burn_header_hash, block_height, op, status FROM sbtc_ops WHERE status=?1 ORDER BY block_height, op ASC
        "#
    }

    const fn sql_select_pk() -> &'static str {
        r#"
        SELECT txid, burn_header_hash, block_height, op, status FROM sbtc_ops WHERE txid=?1 AND burn_header_hash=?2
        "#
    }

    const fn sql_select_max_burn_height() -> &'static str {
        r#"
        SELECT MAX(block_height) FROM sbtc_ops
        "#
    }
}

impl PegQueue for SqlitePegQueue {
    type Error = Error;

    fn sbtc_op(&self) -> Result<Option<SbtcOp>, Self::Error> {
        let maybe_entry = self.get_single_entry_with_status(&Status::New)?;

        let Some(mut entry) = maybe_entry else {
            return Ok(None)
        };

        entry.status = Status::Pending;
        self.insert(&entry)?;

        Ok(Some(entry.op))
    }

    fn poll<N: stacks_node::StacksNode>(&self, stacks_node: &N) -> Result<(), Self::Error> {
        let target_block_height = stacks_node.burn_block_height();

        for block_height in (self.max_observed_block_height()? + 1)..=target_block_height {
            for peg_in_op in stacks_node.get_peg_in_ops(block_height) {
                let entry = Entry {
                    block_height,
                    status: Status::New,
                    txid: peg_in_op.txid,
                    burn_header_hash: peg_in_op.burn_header_hash,
                    op: SbtcOp::PegIn(peg_in_op),
                };

                self.insert(&entry)?;
            }

            for peg_out_request_op in stacks_node.get_peg_out_request_ops(block_height) {
                let entry = Entry {
                    block_height,
                    status: Status::New,
                    txid: peg_out_request_op.txid,
                    burn_header_hash: peg_out_request_op.burn_header_hash,
                    op: SbtcOp::PegOutRequest(peg_out_request_op),
                };

                self.insert(&entry)?;
            }
        }

        Ok(())
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

#[derive(Debug)]
struct Entry {
    burn_header_hash: BurnchainHeaderHash,
    txid: Txid,
    block_height: u64,
    op: SbtcOp,
    status: Status,
}

impl Entry {
    fn from_row(row: &SqliteRow) -> Result<Self, SqliteError> {
        let txid = Txid::from_hex(&row.get::<_, String>(0)?).map_err(Error::from)?;

        let burn_header_hash =
            BurnchainHeaderHash::from_hex(&row.get::<_, String>(1)?).map_err(Error::from)?;

        let block_height = row.get::<_, i64>(2)? as u64; // Stacks will crash before the coordinator if this is invalid

        let op: SbtcOp = serde_json::from_str(&row.get::<_, String>(3)?).map_err(Error::from)?;

        let status: Status = row.get::<_, String>(4)?.parse()?;

        Ok(Self {
            burn_header_hash,
            txid,
            block_height,
            op,
            status,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
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
#[cfg(test)]
mod tests {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    use blockstack_lib::{
        chainstate::stacks::address::PoxAddress,
        types::chainstate::StacksAddress,
        util::{hash::Hash160, secp256k1::MessageSignature},
    };

    use crate::peg_queue::PegQueue;

    use super::*;

    #[test]
    fn calling_sbtc_op_should_return_new_peg_ops() {
        let peg_queue = SqlitePegQueue::in_memory(0).unwrap();
        let number_of_simulated_blocks: u64 = 3;

        let stacks_node_mock = default_stacks_node_mock(number_of_simulated_blocks);

        // No ops before polling
        assert!(peg_queue.sbtc_op().unwrap().is_none());

        // Should cause the peg_queue to fetch 3 peg in ops
        peg_queue.poll(&stacks_node_mock).unwrap();

        for height in 1..=number_of_simulated_blocks {
            let next_op = peg_queue.sbtc_op().unwrap().unwrap();
            assert!(next_op.as_peg_in().is_some());
            assert_eq!(next_op.as_peg_in().unwrap().block_height, height);

            let next_op = peg_queue.sbtc_op().unwrap().unwrap();
            assert!(next_op.as_peg_out_request().is_some());
            assert_eq!(next_op.as_peg_out_request().unwrap().block_height, height);
        }
    }

    #[test]
    fn calling_poll_should_not_query_new_ops_if_at_block_height() {
        let peg_queue = SqlitePegQueue::in_memory(0).unwrap();
        let number_of_simulated_blocks: u64 = 3;

        let stacks_node_mock = default_stacks_node_mock(number_of_simulated_blocks);

        // Fast forward past first poll
        peg_queue.poll(&stacks_node_mock).unwrap();
        for _ in 1..=number_of_simulated_blocks {
            peg_queue.sbtc_op().unwrap().unwrap();
            peg_queue.sbtc_op().unwrap().unwrap();
        }

        let mut stacks_node_mock = stacks_node::MockStacksNode::new();

        stacks_node_mock
            .expect_burn_block_height()
            .return_const(number_of_simulated_blocks);

        stacks_node_mock.expect_get_peg_in_ops().never();
        stacks_node_mock.expect_get_peg_out_request_ops().never();

        peg_queue.poll(&stacks_node_mock).unwrap();
    }

    #[test]
    fn calling_poll_should_find_new_ops_if_at_new_block_height() {
        let peg_queue = SqlitePegQueue::in_memory(0).unwrap();
        let number_of_simulated_blocks: u64 = 3;
        let number_of_simulated_blocks_second_poll: u64 = 5;

        let stacks_node_mock = default_stacks_node_mock(number_of_simulated_blocks);

        // Fast forward past first poll
        peg_queue.poll(&stacks_node_mock).unwrap();
        for _ in 1..=number_of_simulated_blocks {
            peg_queue.sbtc_op().unwrap().unwrap();
            peg_queue.sbtc_op().unwrap().unwrap();
        }

        let stacks_node_mock = default_stacks_node_mock(number_of_simulated_blocks_second_poll);
        peg_queue.poll(&stacks_node_mock).unwrap();

        for height in number_of_simulated_blocks + 1..=number_of_simulated_blocks_second_poll {
            let next_op = peg_queue.sbtc_op().unwrap().unwrap();
            assert!(next_op.as_peg_in().is_some());
            assert_eq!(next_op.as_peg_in().unwrap().block_height, height);

            let next_op = peg_queue.sbtc_op().unwrap().unwrap();
            assert!(next_op.as_peg_out_request().is_some());
            assert_eq!(next_op.as_peg_out_request().unwrap().block_height, height);
        }
    }

    #[test]
    fn acknowledged_entries_should_have_acknowledge_status() {
        let peg_queue = SqlitePegQueue::in_memory(0).unwrap();
        let number_of_simulated_blocks: u64 = 1;

        let stacks_node_mock = default_stacks_node_mock(number_of_simulated_blocks);
        peg_queue.poll(&stacks_node_mock).unwrap();

        let next_op = peg_queue.sbtc_op().unwrap().unwrap();
        let peg_in_op = next_op.as_peg_in().unwrap();
        peg_queue
            .acknowledge(&peg_in_op.txid, &peg_in_op.burn_header_hash)
            .unwrap();

        let entry = peg_queue
            .get_entry(&peg_in_op.txid, &peg_in_op.burn_header_hash)
            .unwrap();

        assert_eq!(entry.status, Status::Acknowledged);
    }

    fn default_stacks_node_mock(block_height: u64) -> stacks_node::MockStacksNode {
        let mut stacks_node_mock = stacks_node::MockStacksNode::new();

        stacks_node_mock
            .expect_burn_block_height()
            .return_const(block_height);

        stacks_node_mock
            .expect_get_peg_in_ops()
            .returning(|height| vec![peg_in_op(height)]);

        stacks_node_mock
            .expect_get_peg_out_request_ops()
            .returning(|height| vec![peg_out_request_op(height)]);

        stacks_node_mock
    }

    fn peg_in_op(block_height: u64) -> stacks_node::PegInOp {
        let recipient_stx_addr = StacksAddress::new(26, Hash160([0; 20]));
        let peg_wallet_address =
            PoxAddress::Standard(StacksAddress::new(0, Hash160([0; 20])), None);

        stacks_node::PegInOp {
            recipient: recipient_stx_addr.into(),
            peg_wallet_address,
            amount: 1337,
            memo: vec![1, 3, 3, 7],
            txid: Txid(hash_and_expand(block_height, 1)),
            burn_header_hash: BurnchainHeaderHash(hash_and_expand(block_height, 0)),
            block_height,
            vtxindex: 0,
        }
    }

    fn peg_out_request_op(block_height: u64) -> stacks_node::PegOutRequestOp {
        let recipient_stx_addr = StacksAddress::new(26, Hash160([0; 20]));
        let peg_wallet_address =
            PoxAddress::Standard(StacksAddress::new(0, Hash160([0; 20])), None);

        stacks_node::PegOutRequestOp {
            recipient: PoxAddress::Standard(recipient_stx_addr, None),
            peg_wallet_address,
            amount: 1337,
            fulfillment_fee: 1000,
            signature: MessageSignature([0; 65]),
            memo: vec![1, 3, 3, 7],
            txid: Txid(hash_and_expand(block_height, 2)),
            burn_header_hash: BurnchainHeaderHash(hash_and_expand(block_height, 0)),
            block_height,
            vtxindex: 0,
        }
    }

    fn hash_and_expand(val: u64, nonce: u64) -> [u8; 32] {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(val);
        hasher.write_u64(nonce);
        let hash = hasher.finish();

        hash.to_be_bytes().repeat(4).try_into().unwrap()
    }
}
