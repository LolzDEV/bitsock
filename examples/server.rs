use bitsock::{server::ServerBuilder, Packet};

fn main() {
    let mut server = ServerBuilder::new()
        .port(4444)
        .client_handler(Box::new(|mut c| {
            println!("Client {} connected!", c.address());

            while match c.read() {
                Ok(packet) => {
                    println!("Received: {:?}", packet);
                    c.send(Packet::String("ciao".to_string())).unwrap();
                    true
                }
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
