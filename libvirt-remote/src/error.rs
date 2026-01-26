use crate::protocol;
use serde_xdr;
use std::fmt;
use std::io;
use std::sync::mpsc;

#[derive(Debug)]
pub enum Error {
    DeserializeError(serde_xdr::error::Error),
    ProtocolError(protocol::VirNetMessageError),
    ReceiveError(io::Error),
    ReceiveChannelError(mpsc::RecvTimeoutError),
    SendError(io::Error),
    SerializeError(serde_xdr::error::Error),
    SocketError(io::Error),
    ReceiverNotStartedError,
    ReceiverStopError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "error: {self:?}")
    }
}

impl std::error::Error for Error {}
