use std::io::{Error, ErrorKind, Write};

use crate::{
    http::RequestEx, io_stream::IoStream, state::State, to_io_result::ToIoResult, url::QueryEx,
};

/// The server keeps a state (messages) and can accept and respond to messages using the
/// `update` function.
#[derive(Default)]
pub struct Server(State);

impl Server {
    pub fn update(&mut self, io: &mut impl IoStream) -> Result<(), Error> {
        let rm = io.istream().read_http_request()?;
        let ostream = io.ostream();
        let mut write = |text: &str| ostream.write(text.as_bytes());
        let mut write_line = |line: &str| {
            write(line)?;
            write("\r\n")?;
            Ok::<(), Error>(())
        };
        let mut write_response_line = || write_line("HTTP/1.1 200 OK");
        match rm.method.as_str() {
            "GET" => {
                let query = *rm.url.url_query().get("id").to_io_result("no id")?;
                let msg = self
                    .0
                    .get(query.to_string())
                    .map_or([].as_slice(), |v| v.as_slice());
                let len = msg.len();
                write_response_line()?;
                write_line(format!("content-length:{len}").as_str())?;
                write_line("")?;
                ostream.write(msg)?;
            }
            "POST" => {
                self.0.post(rm.content);
                write_response_line()?;
                write_line("")?;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "unknown HTTP method")),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::MemIoStreamEx;

    use super::Server;

    #[test]
    fn test() {
        let mut server = Server::default();
        {
            const REQUEST: &str = "\
                POST / HTTP/1.1\r\n\
                Content-Length: 6\r\n\
                \r\n\
                Hello!";
            let mut stream = REQUEST.mem_stream();
            server.update(&mut stream).unwrap();
            assert_eq!(stream.i.position(), REQUEST.len() as u64);
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                \r\n";
            assert_eq!(from_utf8(stream.o.get_ref()).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.1\r\n\
                \r\n";
            let mut stream = REQUEST.mem_stream();
            server.update(&mut stream).unwrap();
            assert_eq!(stream.i.position(), REQUEST.len() as u64);
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                content-length:6\r\n\
                \r\n\
                Hello!";
            assert_eq!(from_utf8(stream.o.get_ref()).unwrap(), RESPONSE);
        }
        {
            const REQUEST: &str = "\
                GET /?id=x HTTP/1.1\r\n\
                \r\n";
            let mut stream = REQUEST.mem_stream();
            server.update(&mut stream).unwrap();
            assert_eq!(stream.i.position(), REQUEST.len() as u64);
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                content-length:0\r\n\
                \r\n";
            assert_eq!(from_utf8(stream.o.get_ref()).unwrap(), RESPONSE);
        }
    }
}
