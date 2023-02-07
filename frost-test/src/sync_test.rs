#[cfg(test)]
mod tests {
    // use frost_signer::signing_round::Signer;
    use relay_server::{MemIoStreamEx, Server};

    #[test]
    fn template_test() {
        let mut server = Server::default();
        // let mut signer0 = Signer::default();
        // let mut signer1 = Signer::default();
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
