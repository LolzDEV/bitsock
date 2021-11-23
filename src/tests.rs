use crate::server::ServerBuilder;

#[test]
fn check_server_builder() {
    let server = ServerBuilder::new()
        .address("192.168.1.84")
        .port(8580)
        .build();
    assert_eq!(server.port, 8580);
    assert_eq!(server.address, "192.168.1.84");
}
