#![allow(dead_code)]
extern crate cobs;

use std::net::TcpListener;
use std::thread;
use std::sync::{Arc, RwLock};
use std::clone::Clone;
use std::io::Write;

fn main() {
    eprintln!("Net up.");

    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    let arc_msg: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
    let arc_count: Arc<RwLock<u64>> = Arc::new(RwLock::new(0));

    let arc_msg_w = arc_msg.clone();
    let arc_count_w = arc_count.clone();

    thread::spawn(move || loop {
        let stdin = std::io::stdin();
        let mut msg = String::new();
        let _len = stdin.read_line(&mut msg);
        {
            let mut arc_msg_w = arc_msg_w.write().unwrap();
            let mut arc_count_w = arc_count_w.write().unwrap();
            *arc_msg_w = msg;
            *arc_count_w += 1;
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let arc_msg = arc_msg.clone();
                let arc_count = arc_count.clone();
                thread::spawn(move || {
                    let mut stream = stream;
                    let mut pos = 0;
                    loop {
                        let line = arc_msg.read().unwrap();
                        let count = arc_count.read().unwrap();
                        if pos < *count {
                            let mut line = cobs::encode_vec(line.as_bytes());
                            line.push(0);
                            match stream.write(&line[..]) {
                                Ok(_) => {
                                    stream.flush();
                                }
                                Err(e) => {
                                    eprintln!("Socket Error: {}", e);
                                    return;
                                }
                            }
                            pos = *count;
                        };
                    }
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}
