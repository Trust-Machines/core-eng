use std::io::Error;

use crate::{
    server::{ServerEx, Stream},
    state::State,
};

pub fn run_server<T: Stream>(i: &mut impl Iterator<Item = Result<T, Error>>) {
    let mut state = State::default();
    for stream_or_error in i {
        let f = || stream_or_error?.update_state(&mut state);
        if let Err(e) = f() {
            eprintln!("IO error: {e}");
        }
    }
}
