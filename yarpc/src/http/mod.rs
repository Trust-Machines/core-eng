mod io_stream;
mod mem_io_stream;
mod message;
mod method;
mod request;
mod response;
mod url;

use std::io::Error;

pub use io_stream::IoStream;
pub use mem_io_stream::{MemIoStream, MemIoStreamEx};
pub use message::Message;
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use url::QueryEx;

pub trait Call {
    fn call(&mut self, request: Request) -> Result<Response, Error>;
}
