use std::{collections::HashMap, io::Error};

use super::{message::PROTOCOL, Message, ToIoResult};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Request {
    pub fn new(
        method: String,
        url: String,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Self {
        Self {
            method,
            url,
            protocol: PROTOCOL.to_owned(),
            headers,
            content,
        }
    }
}

impl Message for Request {
    fn new(
        first_line: Vec<String>,
        headers: HashMap<String, String>,
        content: Vec<u8>,
    ) -> Result<Self, Error> {
        let mut i = first_line.into_iter();
        let method = i.next().to_io_result("no method")?;
        let url = i.next().to_io_result("no URL")?;
        let protocol = i.next().to_io_result("no protocol")?;
        Ok(Request {
            method,
            url,
            protocol,
            headers,
            content,
        })
    }

    fn first_line(&self) -> Vec<String> {
        [
            self.method.to_owned(),
            self.url.to_owned(),
            self.protocol.to_owned(),
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

#[cfg(test)]
mod tests {
    use std::{io::Cursor, str::from_utf8};

    use super::{Message, Request};

    #[test]
    fn test() {
        const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Length: 6\r\n\
            \r\n\
            Hello!";
        let mut read = Cursor::new(REQUEST);
        let rm = Request::read(&mut read).unwrap();
        assert_eq!(rm.method, "POST");
        assert_eq!(rm.url, "/");
        assert_eq!(rm.protocol, "HTTP/1.1");
        assert_eq!(rm.headers.len(), 0);
        assert_eq!(from_utf8(&rm.content), Ok("Hello!"));
        assert_eq!(read.position(), REQUEST.len() as u64);
        let mut v = Vec::default();
        rm.write(&mut Cursor::new(&mut v)).unwrap();
        const EXPECTED: &str = "\
            POST / HTTP/1.1\r\n\
            content-length:6\r\n\
            \r\n\
            Hello!";
        assert_eq!(from_utf8(&v), Ok(EXPECTED));
    }

    #[test]
    fn test_header() {
        const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Length: 6\r\n\
            Hello: someThing\r\n\
            \r\n\
            Hello!";
        let mut read = Cursor::new(REQUEST);
        let rm = Request::read(&mut read).unwrap();
        assert_eq!(rm.method, "POST");
        assert_eq!(rm.url, "/");
        assert_eq!(rm.protocol, "HTTP/1.1");
        assert_eq!(rm.headers.len(), 1);
        assert_eq!(rm.headers["hello"], "someThing");
        assert_eq!(from_utf8(&rm.content), Ok("Hello!"));
        assert_eq!(read.position(), REQUEST.len() as u64);
        let mut v = Vec::default();
        rm.write(&mut Cursor::new(&mut v)).unwrap();
        const EXPECTED: &str = "\
            POST / HTTP/1.1\r\n\
            hello:someThing\r\n\
            content-length:6\r\n\
            \r\n\
            Hello!";
        assert_eq!(from_utf8(&v), Ok(EXPECTED));
    }

    #[test]
    fn incomplete_message_test() {
        const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Leng";
        let mut read = Cursor::new(REQUEST);
        assert!(Request::read(&mut read).is_err());
    }

    #[test]
    fn incomplete_content_test() {
        const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Length: 6\r\n\
            \r\n";
        let mut read = Cursor::new(REQUEST);
        assert!(Request::read(&mut read).is_err());
    }

    #[test]
    fn invalid_message_test() {
        const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Length 6\r\n\
            \r\n\
            Hello!";
        let mut read = Cursor::new(REQUEST);
        assert!(Request::read(&mut read).is_err());
    }

    #[test]
    fn no_content_test() {
        const REQUEST: &str = "\
            GET /images/logo.png HTTP/1.1\r\n\
            \r\n";
        let mut read = Cursor::new(REQUEST);
        let rm = Request::read(&mut read).unwrap();
        assert_eq!(rm.method, "GET");
        assert_eq!(rm.url, "/images/logo.png");
        assert_eq!(rm.protocol, "HTTP/1.1");
        assert!(rm.headers.is_empty());
        assert!(rm.content.is_empty());
        assert_eq!(read.position(), REQUEST.len() as u64);
        let mut v = Vec::default();
        rm.write(&mut Cursor::new(&mut v)).unwrap();
        const EXPECTED: &str = "\
            GET /images/logo.png HTTP/1.1\r\n\
            \r\n";
        assert_eq!(from_utf8(&v), Ok(EXPECTED));
    }

    #[test]
    fn invalid_utf8_should_not_panic() {
        const REQUEST: &[u8] = &[0xFF];
        let mut read = Cursor::new(REQUEST);
        let rm = Request::read(&mut read);
        assert!(rm.is_err());
    }
}
