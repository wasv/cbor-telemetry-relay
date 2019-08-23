extern crate serde_cbor;
extern crate serial;

use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
/// Type containing all possible errors from the proxy.
pub enum ProxyError {
    /// Thown when error encountered with serial port.
    IoError(io::Error),
    /// Thrown when error occurs while decoding COBS buffer.
    DecodeError,
    /// Thrown when error occurs while deserializing buffer to a CBOR value.
    UnpackError(serde_cbor::error::Error),
    /// Errors without a known category or cause.
    InternalError,
    /// Thrown when error occurs while parsing CBOR value to Frame.
    ParseError(String),
    /// Thrown when data read from serial port timesout.
    TimedOut,
    /// Thrown when serial device disconnects from computer.
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

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ProxyError::IoError(ref err) => Some(err),
            ProxyError::UnpackError(ref err) => Some(err),
            // Our custom error doesn't have an underlying cause.
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

/// Convert from io::Errors to ProxyErrors.
/// Changes io Timeouts to ProxyError::TimedOuts and EOFs to Disconnects.
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
