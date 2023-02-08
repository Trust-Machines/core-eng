use std::collections::HashMap;

use super::{common::{Common, PROTOCOL}, message::Message};

pub struct Response {
    code: u16,
    phrase: String,
    common: Common,
}

impl Response {
    pub fn new(code: u16, phrase: String, headers: HashMap<String, String>, content: Vec<u8>) -> Self {
        Self {
            code,
            phrase,
            common: Common {
                protocol: PROTOCOL.to_string(),
                headers,
                content
            },
        }
    }
}

impl Message for Response {
    fn first_line(&self) -> String {
        self.common.protocol.to_owned()
            + " "
            + self.code.to_string().as_str()
            + " "
            + self.phrase.as_str()
    }

    fn common(&self) -> &Common {
        &self.common
    }
}
