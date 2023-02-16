use std::io::{Error, ErrorKind, Write};

use crate::{
    http::{Message, Request, Response, ToIoResult},
    io_stream::IoStream,
    mem_io_stream::MemIoStreamEx,
    mem_state::MemState,
    state::State,
    url::QueryEx,
};

/// The server keeps a state (messages) and can accept and respond to messages using the
/// `update` function.
///
/// ## Example
///
/// ```
/// let mut server = relay_server::Server::default();
/// // send a message using a bidirectional stream.
/// {
///   const REQUEST: &str = "\
///     POST / HTTP/1.0\r\n\
///     Content-Length: 6\r\n\
///     \r\n\
///     Hello!";
///   let response = server.call(REQUEST.as_bytes()).unwrap();
///   const RESPONSE: &str = "\
///     HTTP/1.0 200 OK\r\n\
///     \r\n";
///   assert_eq!(std::str::from_utf8(&response).unwrap(), RESPONSE);
/// }
/// ```
#[derive(Default)]
pub struct Server(MemState);

impl Server {
    pub fn update(&mut self, io: &mut impl IoStream) -> Result<(), Error> {
        let request = Request::read(io.istream())?;
        let ostream = io.ostream();

        let content = match request.method.as_str() {
            "GET" => {
                let query = *request.url.url_query().get("id").to_io_result("no id")?;
                self.0.get(query.to_string())
            }
            "POST" => {
                self.0.post(request.content);
                Vec::default()
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "unknown HTTP method")),
        };
        let response = Response::new(200, "OK".to_string(), Default::default(), content);
        response.write(ostream)?;
        ostream.flush()?;
        Ok(())
    }
    // TODO: move this function to a `test` mod.
    pub fn call(&mut self, msg: &[u8]) -> Result<Vec<u8>, Error> {
        let mut result = Vec::default();
        let mut stream = msg.mem_io_stream(&mut result);
        self.update(&mut stream)?;
        if stream.i.position() != msg.len() as u64 {
            return Err(Error::new(ErrorKind::InvalidData, "invalid request"));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test() {
        let mut server = Server::default();
        {
            const REQUEST: &str = "\
                POST / HTTP/1.0\r\n\
                Content-Length: 6\r\n\
                \r\n\
                Hello!";
            let response = server.call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.0 200 OK\r\n\
                \r\n";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.0\r\n\
                \r\n";
            let response = server.call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.0 200 OK\r\n\
                content-length:6\r\n\
                \r\n\
                Hello!";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.0\r\n\
                \r\n";
            let response = server.call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.0 200 OK\r\n\
                \r\n";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
        // invalid request
        {
            const REQUEST: &str = "\
                POST / HTTP/1.1\r\n\
                Content-Length: 6\r\n\
                \r\n\
                Hello!j";
            let response = server.call(REQUEST.as_bytes());
            assert!(response.is_err());
        }
    }
}
