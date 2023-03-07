use std::collections::HashMap;

use super::{message::PROTOCOL, Request};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Method {
    GET,
    POST,
}

const GET: &str = "GET";
const POST: &str = "POST";

impl Method {
    pub const fn to_str(self) -> &'static str {
        match self {
            Method::GET => GET,
            Method::POST => POST,
        }
    }
    pub fn try_parse(value: &str) -> Option<Self> {
        match value {
            GET => Some(Self::GET),
            POST => Some(Self::POST),
            _ => None,
        }
    }
    pub fn request(
        self,
        url: String,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Request {
        Request {
            method: self,
            url,
            protocol: PROTOCOL.to_owned(),
            headers,
            content,
        }
    }
}
