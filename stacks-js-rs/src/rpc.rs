use std::io;

use serde::{de::DeserializeOwned, Serialize};

/// RPC (Remoter Procedure Call)
pub trait Rpc {
    fn call<I: Serialize, O: DeserializeOwned>(&mut self, input: &I) -> io::Result<O>;
}
