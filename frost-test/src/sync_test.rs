#[cfg(test)]
mod tests {
    use relay_server::{Server, MemIoStreamEx};

    #[test]
    fn simple_test() {
        let mut server = Server::default();
        // send a message using a bidirectional stream.
        {
            const REQUEST: &str = "\
            POST / HTTP/1.1\r\n\
            Content-Length: 6\r\n\
            \r\n\
            Hello!";
            let mut stream = REQUEST.mem_io_stream();
            server.update(&mut stream);
            // stream.o
        }
    }
}
