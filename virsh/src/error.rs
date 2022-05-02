use std::io;
use std::string;

#[derive(Debug)]
pub enum Error {
    Encoding(string::FromUtf8Error),
    Io(io::Error),
    Libvirt(Box<libvirt_remote::error::Error>),
    NotSupported,
    Uri(url::ParseError),
}

impl From<string::FromUtf8Error> for Error {
    fn from(error: string::FromUtf8Error) -> Self {
        Error::Encoding(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<libvirt_remote::error::Error> for Error {
    fn from(error: libvirt_remote::error::Error) -> Self {
        Error::Libvirt(Box::new(error))
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Uri(error)
    }
}
