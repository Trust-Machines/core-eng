use std::io;

use serde::{de::DeserializeOwned, Serialize};

/// RPC (Remote Procedure Call)
pub trait Rpc {
    fn call<I: Serialize, O: Serialize + DeserializeOwned>(&mut self, input: &I) -> io::Result<O>;
}
