use crate::{
    http::{Request, Response},
    state::State,
};

pub struct RemoteState<T: FnMut(Request) -> Response>(pub T);

impl<T: FnMut(Request) -> Response> State for RemoteState<T> {
    fn get(&mut self, node_id: String) -> Vec<u8> {
        let request = Request::new(
            "GET".to_string(),
            format!("/?id={node_id}"),
            Default::default(),
            Default::default(),
        );
        self.0(request).content
    }

    fn post(&mut self, msg: Vec<u8>) {
        let request = Request::new("POST".to_string(), "/".to_string(), Default::default(), msg);
        self.0(request);
    }
}
