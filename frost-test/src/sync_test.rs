#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use frost_signer::signing_round::SigningRound;
    use relay_server::Server;

    #[test]
    fn template_test() {
        let mut server = Server::default();
        let _signers = [
            SigningRound::new(7, 10, 0, [0, 1].to_vec()),
            SigningRound::new(7, 10, 0, [2, 3].to_vec()),
            SigningRound::new(7, 10, 0, [4, 5, 6, 7, 8].to_vec()),
            SigningRound::new(7, 10, 0, [10].to_vec()),
        ];
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
    }
}
