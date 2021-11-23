use std::{io::Write, net::TcpStream};

use crate::{ConnectionError, Packet};

/// Logical server data structure.
pub struct LogicalServer {
    address: String,
    stream: TcpStream,
}

/// Physical client data structure.
pub struct Client {
    connection: LogicalServer,
}

impl Client {
    pub fn connect(address: &str, port: u16) -> Result<Self, ConnectionError> {
        match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => Ok(Self {
                connection: LogicalServer {
                    address: stream.local_addr().unwrap().to_string(),
                    stream,
                },
            }),
            Err(e) => Err(ConnectionError::Client(e.to_string())),
        }
    }

    pub fn send(mut self, packet: Packet) -> Result<usize, std::io::Error> {
        self.connection.stream.write(packet.encode().as_slice())
    }
}
