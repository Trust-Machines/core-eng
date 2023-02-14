use std::str::FromStr;

use blockstack_lib::burnchains::Txid;
use blockstack_lib::types::chainstate::BurnchainHeaderHash;

use crate::peg_queue;
use crate::stacks_node;

pub struct SqlitePegQueue {
    conn: rusqlite::Connection,
}

impl peg_queue::PegQueue for SqlitePegQueue {
    type Error = peg_queue::Error;

    fn sbtc_op(&self) -> Result<Option<peg_queue::SbtcOp>, Self::Error> {
        let maybe_entry = self.get_singe_entry_with_status(&Status::New)?;

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
                    op: peg_queue::SbtcOp::PegIn(peg_in_op),
                };

                self.insert(&entry)?;
            }

            for peg_out_request_op in stacks_node.get_peg_out_request_ops(block_height) {
                let entry = Entry {
                    block_height,
                    status: Status::New,
                    txid: peg_out_request_op.txid,
                    burn_header_hash: peg_out_request_op.burn_header_hash,
                    op: peg_queue::SbtcOp::PegOutRequest(peg_out_request_op),
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

impl SqlitePegQueue {
    pub fn in_memory() -> Result<Self, peg_queue::Error> {
        let conn = rusqlite::Connection::open_in_memory()?;
        let self_ = Self { conn };
        self_.initialize()?;
        Ok(self_)
    }

    fn initialize(&self) -> Result<(), peg_queue::Error> {
        self.conn.execute(Self::sql_schema(), rusqlite::params![])?;

        Ok(())
    }

    fn insert(&self, entry: &Entry) -> Result<(), peg_queue::Error> {
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

    fn get_singe_entry_with_status(
        &self,
        status: &Status,
    ) -> Result<Option<Entry>, peg_queue::Error> {
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
    ) -> Result<Entry, peg_queue::Error> {
        Ok(self.conn.prepare(Self::sql_select_pk())?.query_row(
            rusqlite::params![txid.to_hex(), burn_header_hash.to_hex()],
            Entry::from_row,
        )?)
    }

    fn max_observed_block_height(&self) -> Result<u64, peg_queue::Error> {
        Ok(self
            .conn
            .query_row(
                Self::sql_select_max_burn_height(),
                rusqlite::params![],
                |row| row.get::<_, i64>(0),
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

            PRIMARY_KEY(txid, burn_header_hash),
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
        SELECT (txid, burn_header_hash, op, status) FROM sbtc_ops WHERE status=?1
        "#
    }

    const fn sql_select_pk() -> &'static str {
        r#"
        SELECT (txid, burn_header_hash, op, status) FROM sbtc_ops WHERE txid=?1 AND burn_header_hash=?2
        "#
    }

    const fn sql_select_max_burn_height() -> &'static str {
        r#"
        SELECT MAX(burn_height) FROM sbtc_ops
        "#
    }
}

struct Entry {
    burn_header_hash: BurnchainHeaderHash,
    txid: Txid,
    block_height: u64,
    op: peg_queue::SbtcOp,
    status: Status,
}

impl Entry {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let burn_header_hash = BurnchainHeaderHash::from_hex(&row.get::<_, String>(0)?)
            .map_err(peg_queue::Error::from)?;
        let txid = Txid::from_hex(&row.get::<_, String>(1)?).map_err(peg_queue::Error::from)?;
        let block_height = row.get::<_, i64>(2)? as u64; // Stacks will crash before the coordinator if this is invalid
        let op: peg_queue::SbtcOp =
            serde_json::from_str(&row.get::<_, String>(3)?).map_err(peg_queue::Error::from)?;
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
    type Err = peg_queue::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "new" => Self::New,
            "pending" => Self::Pending,
            "acknowledged" => Self::Acknowledged,
            other => return Err(peg_queue::Error::UnrecognizedStatusString(other.to_owned())),
        })
    }
}
