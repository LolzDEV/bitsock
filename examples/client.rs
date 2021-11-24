use bitsock::{client::Client, Packet};

fn main() {
    let mut client = Client::connect("0.0.0.0", 4444).unwrap();
    if let Err(_) = client.send(Packet::I32(5)) {
        eprintln!("Failed to send packet");
    } else {
        let data = client.read().unwrap();
        println!("Received {:?}!", data);
    }
    loop {}
}
