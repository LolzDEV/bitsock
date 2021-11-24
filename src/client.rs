use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::{ConnectionError, Packet, ReadingError};

/// Physical client data structure.
pub struct Client {
    stream: TcpStream,
}

impl Client {
    /// Connect the client to a server with given ip and port and return the client object.
    pub fn connect(address: &str, port: u16) -> Result<Self, ConnectionError> {
        match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => Ok(Self { stream }),
            Err(e) => Err(ConnectionError::Client(e.to_string())),
        }
    }

    /// Send a [Packet] to the server.
    pub fn send(&mut self, packet: Packet) -> Result<usize, std::io::Error> {
        println!("{:?}", packet.encode().as_slice());

        self.stream.write(packet.encode().as_slice())
    }

    /// Listen to a [Packet] from the server.
    pub fn read(&mut self) -> Result<Packet, ReadingError> {
        let mut data = [0 as u8; 50];

        match self.stream.read(&mut data) {
            Ok(_) => {
                println!("{:?}", data);
                if let Ok(packet) = Packet::decode(data.to_vec()) {
                    Ok(packet)
                } else {
                    Err(ReadingError::Decode)
                }
            }
            Err(_) => Err(ReadingError::Reading),
        }
    }

    /// Close the connection with the client.
    pub fn disconnect(&self) -> Result<(), std::io::Error> {
        self.stream.shutdown(Shutdown::Both)?;

        Ok(())
    }
}
