pub trait State {
    fn get(&mut self, node_id: String) -> Vec<u8>;
    fn post(&mut self, msg: Vec<u8>);
}
