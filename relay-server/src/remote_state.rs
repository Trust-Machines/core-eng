use crate::{
    http::{Message, Request, Response},
    mem_state::State,
    IoStream,
};

struct RemoteState<T: IoStream>(T);

impl<T: IoStream> RemoteState<T> {
    fn remote_call(&mut self, request: Request) -> Response {
        request.write(self.0.ostream()).unwrap();
        Response::read(self.0.istream()).unwrap()
    }
}

impl<T: IoStream> State for RemoteState<T> {
    fn get(&mut self, node_id: String) -> Vec<u8> {
        let request = Request::new(
            "GET".to_string(),
            format!("/?id={node_id}"),
            Default::default(),
            Default::default(),
        );
        self.remote_call(request).content
    }

    fn post(&mut self, msg: Vec<u8>) {
        let request = Request::new(
            "POST".to_string(),
            "/".to_string(),
            Default::default(),
            msg,
        );
        self.remote_call(request);
    }
}
