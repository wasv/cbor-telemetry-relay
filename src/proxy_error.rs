extern crate serde_cbor;
extern crate serial;

use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ProxyError {
    IoError(io::Error),
    UnpackError(serde_cbor::error::Error),
    DecodeError,
    InternalError,
    ParseError(String),
    TimedOut,
    Disconnect,
}

impl Error for ProxyError {
    fn description(&self) -> &str {
        match *self {
            ProxyError::IoError(ref err) => err.description(),
            ProxyError::UnpackError(ref err) => err.description(),
            ProxyError::DecodeError => "CBOR Decoding Error",
            ProxyError::InternalError => "Internal Program Error",
            ProxyError::ParseError(ref msg) => msg,
            ProxyError::TimedOut => "Read timed out.",
            ProxyError::Disconnect => "Port Disconnected",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ProxyError::IoError(ref err) => Some(err),
            ProxyError::UnpackError(ref err) => Some(err),
            // Our custom error doesn't have an underlying cause,
            // but we could modify it so that it does.
            ProxyError::ParseError(_) => None,
            ProxyError::DecodeError => None,
            ProxyError::InternalError => None,
            ProxyError::TimedOut => None,
            ProxyError::Disconnect => None,
        }
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProxyError::IoError(ref err) => write!(f, "IO Error: {}", err),
            ProxyError::UnpackError(ref err) => write!(f, "Unpack Error: {}", err),
            ProxyError::ParseError(ref msg) => write!(f, "Parse Error: {}", msg),
            ProxyError::DecodeError => write!(f, "COBS Decoding Error"),
            ProxyError::InternalError => write!(f, "Internal Program Error"),
            ProxyError::TimedOut => write!(f, "Read timed out."),
            ProxyError::Disconnect => write!(f, "Port Disconnected"),
        }
    }
}

impl From<io::Error> for ProxyError {
    fn from(err: io::Error) -> ProxyError {
        match err.kind() {
            io::ErrorKind::UnexpectedEof => ProxyError::Disconnect,
            io::ErrorKind::TimedOut => ProxyError::TimedOut,
            _ => ProxyError::IoError(err),
        }
    }
}

impl From<serial::Error> for ProxyError {
    fn from(_err: serial::Error) -> ProxyError {
        ProxyError::Disconnect
    }
}

impl From<serde_cbor::error::Error> for ProxyError {
    fn from(err: serde_cbor::error::Error) -> ProxyError {
        if err.is_eof() {
            return ProxyError::Disconnect;
        }
        ProxyError::UnpackError(err)
    }
}
