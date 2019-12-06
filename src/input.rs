extern crate serial;

use std::os::unix::net::UnixStream;
use std::process;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use input::serial::prelude::*;

/// Serial Port settings. Defaults are fine, sinc$e BluePill autonegottaiates.
const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub struct Input<'a> {
    pub source: Box<dyn BufRead + 'a>,
}

/// Possible methods for creating an Input stream from various sources.
impl<'a> Input<'a> {
    pub fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input {
            source: Box::new(stdin.lock()),
        }
    }

    pub fn file(path: &str) -> io::Result<Input<'a>> {
        File::open(path).map(|file| Input {
            source: Box::new(io::BufReader::new(file)),
        })
    }

    pub fn serial_port(path: &str) -> Input<'a> {
        eprintln!("Opening port: {}", path);
        let mut port = serial::open(&path).unwrap_or_else(|e| {
            eprintln!("Error opening port: {}", e.to_string());
            process::exit(-1);
        });
        port.configure(&SETTINGS).unwrap_or_else(|e| {
            eprintln!("Error configuring port: {}", e.to_string());
            process::exit(-1);
        });
        Input {
            source: Box::new(BufReader::new(port)),
        }
    }

    pub fn socket(path: &str) -> Input<'a> {
        eprintln!("Opening socket: {}", path);
        let socket = UnixStream::connect(path).unwrap_or_else(|e| {
            eprintln!("Error opening socket: {}", e.to_string());
            process::exit(-1);
        });
        Input {
            source: Box::new(BufReader::new(socket)),
        }
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}
