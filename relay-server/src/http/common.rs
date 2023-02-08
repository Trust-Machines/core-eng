use std::{collections::HashMap};

#[derive(Debug)]
pub struct Common {
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

pub const PROTOCOL: &str = "HTTP/1.1"; 