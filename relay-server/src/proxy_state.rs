use std::io::Error;

use yarpc::http::{Call, Method};

use crate::state::State;

pub struct ProxyState<T: Call>(pub T);

impl<T: Call> State for ProxyState<T> {
    fn get(&mut self, node_id: String) -> Result<Vec<u8>, Error> {
        Ok(self
            .0
            .call(Method::GET.request(
                format!("/?id={node_id}"),
                Default::default(),
                Default::default(),
            ))?
            .content)
    }

    fn post(&mut self, msg: Vec<u8>) -> Result<(), Error> {
        self.0
            .call(Method::POST.request("/".to_string(), Default::default(), msg))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test() {
        let mut state = ProxyState(Server::default());
        assert!(state.get(1.to_string()).unwrap().is_empty());
        assert!(state.get(3.to_string()).unwrap().is_empty());
        // assert_eq!(0, state.highwaters.len());
        state.post("Msg # 0".as_bytes().to_vec()).unwrap();
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(1.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(5.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(4.to_string()).unwrap()
        );
        assert!(state.get(1.to_string()).unwrap().is_empty());
        state.post("Msg # 1".as_bytes().to_vec()).unwrap();
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(1.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(3.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(5.to_string()).unwrap()
        );
        state.post("Msg # 2".as_bytes().to_vec()).unwrap();
        assert_eq!(
            "Msg # 2".as_bytes().to_vec(),
            state.get(1.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(4.to_string()).unwrap()
        );
        assert_eq!(
            "Msg # 2".as_bytes().to_vec(),
            state.get(4.to_string()).unwrap()
        );
    }
}
