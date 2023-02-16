use std::{collections::HashMap, io::Read};

use super::{
    message::{Message, PROTOCOL},
    ToIoResult,
};

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub code: u16,
    pub phrase: String,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Response {
    pub fn new(
        code: u16,
        phrase: String,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Self {
        Self {
            protocol: PROTOCOL.to_owned(),
            code,
            phrase,
            headers,
            content,
        }
    }
}

impl Message for Response {
    fn new(
        first_line: Vec<String>,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Result<Self, std::io::Error> {
        let mut i = first_line.into_iter();
        let protocol = i.next().to_io_result("expecting protocol")?;
        let code = i
            .next()
            .to_io_result("expecting status code")?
            .parse()
            .to_io_result("invalid status code")?;
        let phrase = i.next().unwrap_or(String::default());
        Ok(Response {
            protocol,
            code,
            phrase,
            headers,
            content,
        })
    }

    fn first_line(&self) -> Vec<String> {
        [
            self.protocol.clone(),
            self.code.to_string(),
            self.phrase.clone(),
        ]
        .to_vec()
    }

    fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    fn content(&self) -> &Vec<u8> {
        &self.content
    }
}

pub trait ResponseEx: Read {}
