use std::io::{Cursor, Error, Write};

use yarpc::{
    http::{Call, IoStream, MemIoStreamEx, Message, Method, QueryEx, Request, Response},
    to_io_result::{err, ToIoResult},
};

use crate::{mem_state::MemState, state::State};

/// The server keeps a state (messages) and can accept and respond to messages using the
/// `update` function.
///
/// ## Example
///
/// ```
/// use relay_server::Server;
/// use yarpc::http::{Call, Method, Response};
///
/// let mut server = Server::default();
/// // send a message "Hello!"
/// {
///     let request = Method::POST.request(
///         "/".to_string(),
///         Default::default(),
///         "Hello!".as_bytes().to_vec(),
///     );
///     let response = server.call(request).unwrap();
///     let expected = Response::new(
///         200,
///         "OK".to_string(),
///         Default::default(),
///         Default::default(),
///     );
///     assert_eq!(response, expected);
/// }
/// ```
#[derive(Default)]
pub struct Server(MemState);

impl Server {
    pub fn update(&mut self, io: &mut impl IoStream) -> Result<(), Error> {
        let request = Request::read(io.istream())?;
        let ostream = io.ostream();

        let content = match request.method {
            Method::GET => {
                let query = *request.url.url_query().get("id").to_io_result("no id")?;
                self.0.get(query.to_string())?
            }
            Method::POST => {
                self.0.post(request.content)?;
                Vec::default()
            }
        };
        let response = Response::new(200, "OK".to_string(), Default::default(), content);
        response.write(ostream)?;
        ostream.flush()?;
        Ok(())
    }
    fn raw_call(&mut self, msg: &[u8]) -> Result<Vec<u8>, Error> {
        let mut result = Vec::default();
        let mut stream = msg.mem_io_stream(&mut result);
        self.update(&mut stream)?;
        if stream.i.position() != msg.len() as u64 {
            return err("invalid request");
        }
        Ok(result)
    }
}

impl Call for Server {
    fn call(&mut self, request: Request) -> Result<Response, Error> {
        let response_buf = {
            let mut request_stream = Cursor::<Vec<u8>>::default();
            request.write(&mut request_stream)?;
            self.raw_call(request_stream.get_ref())?
        };
        Response::read(&mut Cursor::new(response_buf))
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
            let response = server.raw_call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                \r\n";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.0\r\n\
                \r\n";
            let response = server.raw_call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                content-length:6\r\n\
                \r\n\
                Hello!";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.1\r\n\
                \r\n";
            let response = server.raw_call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
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
            let response = server.raw_call(REQUEST.as_bytes());
            assert!(response.is_err());
        }
    }
}
