/*
Example: Logging

This is a simple server application with custom logging, it can communicate with the Client example.
*/

use bitsock::{server::ServerBuilder, LogStage, Packet};

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
        // Setup the custom logger
        .log_handler(Box::new(|stage, level, message| {
            if let LogStage::SERVER = stage {
                match level {
                    bitsock::LogLevel::INFO => println!("[SERVER][INFO]: {}", message),
                    bitsock::LogLevel::WARN => println!("[SERVER][WARNING]: {}", message),
                    bitsock::LogLevel::ERROR => println!("[SERVER][ERROR]: {}", message),
                }
            }
        }))
        .build();

    server.run();
}
