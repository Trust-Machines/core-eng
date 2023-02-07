use crate::{IoStream, mem_state::State};

struct RemoteState<T: IoStream>(T);

impl<T: IoStream> State for RemoteState<T> {
    fn get(&mut self, node_id: String) -> Option<&Vec<u8>> {
        let o = self.0.ostream();
        todo!()
    }

    fn post(&mut self, msg: Vec<u8>) {
        todo!()
    }
}