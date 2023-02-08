use std::collections::HashMap;

pub trait State {
    fn get(&mut self, node_id: String) -> Vec<u8>;
    fn post(&mut self, msg: Vec<u8>);
}

#[derive(Default)]
pub struct MemState {
    /// The value for this map is an index for the last read message for this node.
    highwaters: HashMap<String, usize>,
    queue: Vec<Vec<u8>>,
}

impl State for MemState {
    fn get(&mut self, node_id: String) -> Vec<u8> {
        let first_unread = self
            .highwaters
            .get(&node_id)
            .map_or(0, |last_read| *last_read + 1);
        let result = self.queue.get(first_unread);
        if let Some(r) = result {
            self.highwaters.insert(node_id, first_unread);
            r.clone()
        } else {
            Vec::default()
        }
    }
    fn post(&mut self, msg: Vec<u8>) {
        self.queue.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::{MemState, State};
    #[test]
    fn state_test() {
        let mut state = MemState::default();
        assert!(state.get(1.to_string()).is_empty());
        assert!(state.get(3.to_string()).is_empty());
        assert_eq!(0, state.highwaters.len());
        state.post("Msg # 0".as_bytes().to_vec());
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(1.to_string())
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(5.to_string())
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(4.to_string())
        );
        assert!(state.get(1.to_string()).is_empty());
        state.post("Msg # 1".as_bytes().to_vec());
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(1.to_string())
        );
        assert_eq!(
            "Msg # 0".as_bytes().to_vec(),
            state.get(3.to_string())
        );
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(5.to_string())
        );
        state.post("Msg # 2".as_bytes().to_vec());
        assert_eq!(
            "Msg # 2".as_bytes().to_vec(),
            state.get(1.to_string())
        );
        assert_eq!(
            "Msg # 1".as_bytes().to_vec(),
            state.get(4.to_string())
        );
        assert_eq!(
            "Msg # 2".as_bytes().to_vec(),
            state.get(4.to_string())
        );
    }
}
