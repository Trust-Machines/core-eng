use std::io::Error;

/// An interface that defines the functions that need to be implemented in order to store and
/// retrieve messages.
///
/// This trait provides a way for the relay-server to abstract away the underlying storage
/// mechanism and allows for different storage implementations to be used, such as in-memory
/// or a remote database. By implementing the State trait, you can customize the storage
/// mechanism to fit your specific use case.
pub trait State {
    fn get(&mut self, node_id: String) -> Result<Vec<u8>, Error>;
    fn post(&mut self, msg: Vec<u8>) -> Result<(), Error>;
}
