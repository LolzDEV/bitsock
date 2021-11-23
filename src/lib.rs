use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub mod client;
pub mod server;

pub enum ReadingError {
    /// Error returned when the server fails to read data from the client.
    Reading,

    /// Error returned when the readed packet fails to be decoded
    Decode,
}

pub enum ConnectionError {
    /// Error returned when a client fail to connect.
    Client(String),
}

/// Error returned when a packet cannot be decoded from [Packet::decode].
pub struct PacketDecodeError;

/// Enum containing all the possible packet types.
#[derive(Serialize, Deserialize)]
pub enum Packet {
    /// Packet containing data in form of bytes.
    Bytes(Vec<u8>),
    /// Packet containing a [String].
    String(String),
    /// Packet containing a [i8].
    I8(i8),
    /// Packet containing a [i16].
    I16(i16),
    /// Packet containing a [i32].
    I32(i32),
    /// Packet containing a [i64].
    I64(i64),
    /// Packet containing a [f32].
    F32(f32),
    /// Packet containing a [f64].
    F64(f64),
    /// Packet containing a [u8].
    U8(u8),
    /// Packet containing a [u16].
    U16(u16),
    /// Packet containing a [u32].
    U32(u32),
    /// Packet containing a [u64].
    U64(u64),
    /// Packet containing data in form of bytes with an identifier which can represent what type of data the packet contains.
    Identified(u32, Vec<u8>),
}

impl Packet {
    pub fn encode(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn decode(bytes: Vec<u8>) -> Result<Self, PacketDecodeError> {
        if let Ok(packet) = bincode::deserialize::<Packet>(bytes.as_slice()) {
            Ok(packet)
        } else {
            Err(PacketDecodeError)
        }
    }
}
