/*
Example: Client

This is a simple client application, it can communicate with the Server example.
*/

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
