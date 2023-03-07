use relay_server::Server;
use yarpc::http::{Call, Method, Response};

#[test]
fn server_test() {
    let mut server = Server::default();
    {
        let request = Method::POST.request(
            "/".to_string(),
            Default::default(),
            "Hello!".as_bytes().to_vec(),
        );
        let response = server.call(request).unwrap();
        let expected = Response::new(
            200,
            "OK".to_string(),
            Default::default(),
            Default::default(),
        );
        assert_eq!(response, expected);
    }
}
