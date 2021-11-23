use bitsock::server::ServerBuilder;

fn main() {
    let mut server = ServerBuilder::new()
        .client_handler(Box::new(|c| {
            println!("Client {} connected!", c.address());
        }))
        .build();

    server.run();
}
