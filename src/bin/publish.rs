extern crate cobs;
extern crate serde_cbor;

use std::thread;
use std::time;
use std::io::Write;
use std::collections::BTreeMap;
use serde_cbor::to_vec;
use serde_cbor::ObjectKey;
use serde_cbor::Value;
use cobs::encode_vec;

fn main() {
    let delay = time::Duration::from_millis(1500);
    let start = time::Instant::now();
    for x in 0..10 {
        let mut frame: BTreeMap<ObjectKey, Value> = BTreeMap::new();
        frame.insert(ObjectKey::String("fnum".to_string()), Value::U64(x));
        frame.insert(
            ObjectKey::String("sender".to_string()),
            Value::String("TEST_SOURCE".to_string()),
        );

        let now = start.elapsed();
        let ts = (now.as_secs() * 1e9 as u64) + (now.subsec_nanos() as u64);
        frame.insert(ObjectKey::String("dt".to_string()), Value::U64(ts));

        frame.insert(ObjectKey::String("ax".to_string()), Value::U64(x * x * x));
        frame.insert(ObjectKey::String("ay".to_string()), Value::U64(x * x));
        frame.insert(ObjectKey::String("az".to_string()), Value::U64(x + x));
        frame.insert(ObjectKey::String("temp".to_string()), Value::F64(3.14159));

        let encoded = to_vec(&frame).unwrap();
        let encoded = encode_vec(&encoded);
        std::io::stdout().write(&encoded).unwrap();
        std::io::stdout().write(b"\0").unwrap();
        std::io::stdout().flush().unwrap();
        thread::sleep(delay);
    }
}
