#![allow(dead_code)]
extern crate cobs;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::BufRead;
use std::process;
use std::env;
use std::string::String;

use cobs::{decode_vec, encode_vec};

use serde_cbor::{from_slice, Value};

mod frame;
use frame::Frame;

mod input;
use input::Input;

mod proxy_error;
use proxy_error::ProxyError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    let stdin = std::io::stdin();
    let mut reader: Input = if argc == 2 {
        input::Input::serial_port(&args[1])
    } else {
        input::Input::console(&stdin)
    };
    eprintln!("Comms up.");
    loop {
        match read_buffer(&mut reader) {
            Ok(val) => match Frame::from_value(val) {
                Ok(frame) => {
                    println!("{}", serde_json::to_string(&frame).unwrap());
                }
                Err(e) => eprintln!("Parse Error: {}", e),
            },
            Err(ProxyError::Disconnect) => {
                eprintln!("Device Disconnected.");
                process::exit(1);
            }
            Err(ProxyError::TimedOut) => {}
            Err(e) => eprintln!("Frame Error: {}", e),
        }
    }
}

fn read_buffer(reader: &mut Input) -> Result<Value, ProxyError> {
    let mut buf: Vec<u8> = Vec::new();
    let num_bytes = reader.source.read_until(0, &mut buf)?;
    if num_bytes == 0 {
        return Err(ProxyError::Disconnect);
    }
    let buf = &buf[0..num_bytes - 1];
    let buf = try!(decode_vec(&buf).map_err(|()| ProxyError::DecodeError));
    let val = try!(from_slice(&buf));
    Ok(val)
}
