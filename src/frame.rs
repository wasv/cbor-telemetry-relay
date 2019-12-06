extern crate serde;
extern crate serde_cbor;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_cbor::{ObjectKey, Value};
use std::time::{SystemTime, UNIX_EPOCH};

use proxy_error::ProxyError::{self, ParseError};

#[derive(Serialize, Debug)]
/// A struct that represents a frame to be sent over the network.
pub struct Frame {
    timestamp: u64,
    sender: String,
    fnum: u64,
    streams: Vec<Stream>,
}

#[derive(Debug)]
/// A sturct for storing the name, and data type associated with a data value.
pub struct Stream {
    name: String,
    value: Data,
    dtype: u64,
}

#[derive(Debug)]
/// An enum for storing the different types of data possible in Streams.
pub enum Data {
    Float(f64),
    Signed(i64),
    Unsigned(u64),
}

/// A custom serializer implementation. Removes Data from containing struct.
/// Also accounts for isssue where 'type' is a reserved word in rust.
impl Serialize for Stream {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Stream", 3)?;
        state.serialize_field("name", &self.name)?;
        match self.value {
            Data::Signed(ref data) => state.serialize_field("value", data)?,
            Data::Unsigned(ref data) => state.serialize_field("value", data)?,
            Data::Float(ref data) => state.serialize_field("value", data)?,
        };
        state.serialize_field("type", &self.dtype)?;
        state.end()
    }
}

impl Frame {
    /// Creates a new Frame from a serde_cbor::Value object or returns a ParseError.
    pub fn from_value(val: serde_cbor::Value) -> Result<Self, ProxyError> {
        let map = val
            .as_object()
            .ok_or_else(|| ParseError("Not a map.".to_string()))?;
        let mut map = map.clone();
        let fnum: Value = map
            .remove(&ObjectKey::String("fnum".to_string()))
            .ok_or_else(|| ParseError("No framenumber.".to_string()))?;
        let fnum: u64 = fnum
            .as_u64()
            .ok_or_else(|| ParseError("Invalid framenumber.".to_string()))?;
        let sender: Value = map
            .remove(&ObjectKey::String("sender".to_string()))
            .ok_or_else(|| ParseError("No sender.".to_string()))?;
        let sender: String = sender
            .as_string()
            .ok_or_else(|| ParseError("Invalid sender.".to_string()))
            .map(|x| x.to_string())?;
        let mut streams: Vec<Stream> = vec![];

        for (key, value) in map {
            let key: String = key
                .as_string()
                .map(|x| x.to_string())
                .ok_or_else(|| ParseError("Invalid key.".to_string()))?;
            match value {
                Value::F64(value) => streams.push(Stream {
                    name: key,
                    value: Data::Float(value),
                    dtype: 0,
                }),
                Value::I64(value) => streams.push(Stream {
                    name: key,
                    value: Data::Signed(value),
                    dtype: 1,
                }),
                Value::U64(value) => streams.push(Stream {
                    name: key,
                    value: Data::Unsigned(value),
                    dtype: 1,
                }),
                _ => return Err(ParseError(format!("Invalid Value for {}", key))),
            };
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = now.as_secs() * (1e9 as u64) + u64::from(now.subsec_nanos());
        Ok(Frame {
            timestamp,
            streams,
            fnum,
            sender,
        })
    }
}
