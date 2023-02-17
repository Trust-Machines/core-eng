use std::io::{Error, ErrorKind};

pub trait ToIoResult {
    type V;
    fn to_io_result(self) -> Result<Self::V, Error>;
}

fn err<T>(msg: &str) -> Result<T, Error> {
    Err(Error::new(ErrorKind::InvalidData, msg))
}

impl<T> ToIoResult for Option<T> {
    type V = T;
    fn to_io_result(self) -> Result<Self::V, Error> {
        self.map_or(err("option"), Ok)
    }
}

impl<T, E> ToIoResult for Result<T, E> {
    type V = T;
    fn to_io_result(self) -> Result<Self::V, Error> {
        self.map_or(err("result"), Ok)
    }
}

pub trait TakeToIoResult {
    type V;
    fn take_to_io_result(&mut self) -> Result<Self::V, Error>;
}

impl<T> TakeToIoResult for Option<T> {
    type V = T;
    fn take_to_io_result(&mut self) -> Result<T, Error> {
        self.take().to_io_result()
    }
}
