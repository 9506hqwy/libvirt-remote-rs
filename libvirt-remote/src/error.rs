use crate::protocol;
use serde_xdr;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    DeserializeError(serde_xdr::error::Error),
    ProtocolError(protocol::VirNetMessageError),
    ReceiveError(io::Error),
    SendError(io::Error),
    SerializeError(serde_xdr::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "error: {self:?}")
    }
}

impl std::error::Error for Error {}
