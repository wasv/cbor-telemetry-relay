# Telemetry Receiver

## Proxy
`src/bin_proxy.rs` contains a network 'proxy' that takes lines from
stdin, COBS encodes the input lines, and transmits the data to an
arbitrary number of connected clients.

## Receiver
`src/bin_receiver.rs` contains a receiver program that takes in CBOR objects
from either stdin or a serial port, checks that the objects are valid
frames, and prints the frames as JSON strings over stdout.

## Publisher
`src/bin_publisher.rs` contains a sample publisher for the system. It emits
frames with a predictable pattern at a regular interval. Used for testing.
