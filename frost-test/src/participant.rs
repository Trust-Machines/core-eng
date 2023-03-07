use crate::event::{Event, Protocol};

pub trait Participant {
    type P: Protocol;
    fn call(&mut self, event: &Event<Self::P>) -> Vec<Event<Self::P>>;
}
