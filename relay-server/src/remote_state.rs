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

#[cfg(test)]
mod tests {
    use super::super::http::*;
    use super::super::*;
    use super::*;

    #[test]
    fn test() {
        use std::io::Cursor;

        let mut server = Server::default();

        let f = |r: Request| {
            let response_buf = {
                let mut request_stream = Cursor::<Vec<u8>>::default();
                r.write(&mut request_stream).unwrap();
                server.call(request_stream.get_ref()).unwrap()
            };
            let mut response_stream = Cursor::new(response_buf);
            Response::read(&mut response_stream).unwrap()
        };

        let mut state = RemoteState(f);
        assert!(state.get(1.to_string()).is_empty());
        assert!(state.get(3.to_string()).is_empty());
        // assert_eq!(0, state.highwaters.len());
        state.post("Msg # 0".as_bytes().to_vec());
        assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(1.to_string()));
        assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(5.to_string()));
        assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(4.to_string()));
        assert!(state.get(1.to_string()).is_empty());
        state.post("Msg # 1".as_bytes().to_vec());
        assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(1.to_string()));
        assert_eq!("Msg # 0".as_bytes().to_vec(), state.get(3.to_string()));
        assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(5.to_string()));
        state.post("Msg # 2".as_bytes().to_vec());
        assert_eq!("Msg # 2".as_bytes().to_vec(), state.get(1.to_string()));
        assert_eq!("Msg # 1".as_bytes().to_vec(), state.get(4.to_string()));
        assert_eq!("Msg # 2".as_bytes().to_vec(), state.get(4.to_string()));
    }
}
