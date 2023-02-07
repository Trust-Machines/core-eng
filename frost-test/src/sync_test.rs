#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    // use frost_signer::signing_round::Signer;
    use relay_server::Server;

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
            let response = server.call(REQUEST.as_bytes()).unwrap();
            const RESPONSE: &str = "\
                HTTP/1.1 200 OK\r\n\
                \r\n";
            assert_eq!(from_utf8(&response).unwrap(), RESPONSE);
        }
    }
}
