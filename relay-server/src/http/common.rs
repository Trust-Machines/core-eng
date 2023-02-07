use std::collections::HashMap;

#[derive(Debug)]
pub struct Common {
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}