# bitsock

[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![Crates.io](https://img.shields.io/crates/v/bitsock)](https://crates.io/crates/bitsock)
[![build](https://github.com/LolzDEV/bitsock/actions/workflows/build.yml/badge.svg)](https://github.com/LolzDEV/bitsock/actions/workflows/build.yml)
[![License](https://img.shields.io/github/license/LolzDEV/bitsock)](LICENSE)
![Lines of code](https://img.shields.io/tokei/lines/github/LolzDEV/bitsock?label=lines%20of%20code)
[![GitHub issues](https://img.shields.io/github/issues/LolzDEV/bitsock)](https://github.com/LolzDEV/bitsock/issues)

Safe Rust crate for creating socket servers and clients with ease.

## Description

This crate can be used for Client <--> Server applications of every purpose, the protocol can be defined by the user thanks to **Packets**:
there are many type of specific purpose builtin Packets and one general purpose Packet type that can be identified with an _u32 id_ (see [Packet](https://docs.rs/bitsock/0.1.0/bitsock/enum.Packet.html)) so that you can create your own protocol.

The client handling has to be done in a simple way: you can specify a _closure_ and that one will be executed on a new _thread_ everytime a client connects.

## Example

_Client_

```rust
use std::time::Duration;

use bitsock::{client::Client, Packet};

fn main() {
    // Create the client object.
    let mut client = Client::connect("0.0.0.0", 4444).unwrap();

    loop {
        // Try to send a packet containing just an i32.
        if let Err(_) = client.send(Packet::I32(5)) {
            eprintln!("Failed to send packet");
        } else {
            // If the packet can be sent, then listen to the server and wait for a Packet.
            let data = client.read().unwrap();

            // If the packet is a string, print it.
            if let Packet::String(s) = data {
                println!("Received String: {}", s);
            } else {
                // If the packet is another type, print the type.
                println!("Received Packet: {:?}", data);
            }
        }

        std::thread::sleep(Duration::from_secs(2));
    }
}
```

_Server_

```rust
use bitsock::{server::ServerBuilder, Packet};

fn main() {
    // Create the server object and bind it to the 4444 port.
    let mut server = ServerBuilder::new()
        .port(4444)
        // Supply a client handler, this will be runned for every connected client (in a new thread).
        .client_handler(Box::new(|mut c| {
            // Print the client address once connected.
            println!("Client {} connected!", c.address());

            // Try to listen for Packet from the Client.
            while match c.read() {
                Ok(packet) => {
                    // Print the received packet
                    println!("Received: {:?}", packet);

                    // Send a string packet to the client.
                    c.send(Packet::String("Hello There!".to_string())).unwrap();
                    true
                }
                // If it fails, disconnect the Client and print the error.
                Err(e) => {
                    c.disconnect().unwrap();
                    println!("Client {} disconnected for {:?}!", c.address(), e);
                    false
                }
            } {}
        }))
        .build();

    server.run();
}
```

## License

See [LICESE](LICENSE)
