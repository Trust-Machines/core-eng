use blockstack_lib::{net::BurnchainOps, types::chainstate::StacksAddress};
use reqwest::blocking::Client;
use serde_json::{from_value, Value};

use crate::stacks_node::{PegInOp, PegOutRequestOp, StacksNode, StacksTransaction};

pub struct NodeClient {
    node_url: String,
    client: Client,
}

impl NodeClient {
    pub fn new(url: &str) -> Self {
        Self {
            node_url: url.to_string(),
            client: Client::new(),
        }
    }

    fn build_url(&self, route: &str) -> String {
        format!("{}{}", self.node_url, route)
    }
}

impl StacksNode for NodeClient {
    fn get_peg_in_ops(&self, block_height: u64) -> Vec<PegInOp> {
        let url = self.build_url(&format!("/v2/burn_ops/peg_in/{}", block_height));

        self.client
            .get(url)
            .send()
            .and_then(|res| res.json::<Value>())
            .map(|json| from_value(json["peg_in"].clone()).unwrap())
            .unwrap()
    }

    fn get_peg_out_request_ops(&self, block_height: u64) -> Vec<PegOutRequestOp> {
        let url = self.build_url(&format!("/v2/burn_ops/peg_out_request/{}", block_height));

        self.client
            .get(url)
            .send()
            .and_then(|res| res.json::<Value>())
            .map(|json| from_value(json["peg_in"].clone()).unwrap())
            .unwrap()
    }

    fn burn_block_height(&self) -> u64 {
        let url = self.build_url("/v2/info");

        self.client
            .get(url)
            .send()
            .and_then(|res| res.json::<Value>())
            .map(|json| json["burn_block_height"].as_u64().unwrap())
            .unwrap()
    }

    // TODO: Figure out what is nonce and how to get it
    fn next_nonce(&self, addr: StacksAddress) {
        todo!()
    }

    // TODO: Find appropriate type for tx
    fn broadcast_transaction(&self, tx: &StacksTransaction) {
        let url = self.build_url("/v2/transactions");

        self.client
            .post(url)
            .json(tx)
            .send()
            .and_then(|res| res.json::<Value>())
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Temporary debugging
    #[test]
    #[ignore]
    fn get_block_height() {
        let client = NodeClient::new("http://localhost:20443");

        dbg!(client.burn_block_height());
    }
}
