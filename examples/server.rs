use bitsock::server::ServerBuilder;

fn main() {
    let mut server = ServerBuilder::new().build();

    server.run();
}
