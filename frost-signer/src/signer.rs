use crate::config::Config;
use crate::net::{HttpNet, HttpNetError, HttpNetListen, Message, Net, NetListen};
use crate::signing_round::SigningRound;
use serde::Deserialize;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use std::{thread, time};

// on-disk format for frost save data
#[derive(Clone, Deserialize, Default, Debug)]
pub struct Signer {
    pub config: Config,
    pub frost_id: u32,
}

impl Signer {
    pub fn new(config: Config, frost_id: u32) -> Self {
        Self { config, frost_id }
    }

    pub fn start_p2p_sync(&mut self) -> Result<(), Error> {
        //Create http relay
        let net: HttpNet = HttpNet::new(self.config.stacks_node_url.clone());
        let net_queue = HttpNetListen::new(net.clone(), vec![]);
        // thread coordination
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        // start p2p sync
        let id = self.frost_id;
        spawn(move || poll_loop(net_queue, tx, id));

        // listen to p2p messages
        self.start_signing_round(&net, rx)
    }

    fn start_signing_round(&self, net: &HttpNet, rx: Receiver<Message>) -> Result<(), Error> {
        let mut round = SigningRound::from(self);

        loop {
            let inbound = rx.recv()?; // blocking
            let outbounds = round.process(inbound.msg).map_err(Error::DKGSigningError)?;
            for out in outbounds {
                let msg = Message {
                    msg: out,
                    sig: [0; 32],
                };
                net.send_message(msg)?;
            }
        }
    }
}

fn poll_loop(mut net: HttpNetListen, tx: Sender<Message>, id: u32) {
    loop {
        net.poll(id);
        match net.next_message() {
            None => {}
            Some(m) => {
                tx.send(m).unwrap();
            }
        };
        thread::sleep(time::Duration::from_millis(500));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http Network Error: {0}")]
    HttpNetError(#[from] HttpNetError),

    #[error("Recv Error: {0}")]
    RecvError(#[from] mpsc::RecvError),

    #[error("Send Error")]
    SendError,

    #[error("DKG signing error")]
    DKGSigningError(String),
}

impl From<mpsc::SendError<Message>> for Error {
    fn from(_: mpsc::SendError<Message>) -> Error {
        Error::SendError
    }
}
