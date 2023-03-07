pub mod dkg;
mod protocol;
pub mod sign;

pub use protocol::Protocol;

pub enum Event<P: Protocol> {
    DkgBegin(dkg::Begin<P>),
    DkgShare(dkg::Share<P>),
    DkgEnd(dkg::End<P>),
    SignBegin(sign::Begin<P>),
    SignNonce(sign::Nonce<P>),
    SignShare(sign::Share<P>),
}
