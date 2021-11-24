use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub mod client;
pub mod server;

#[derive(Debug)]
pub enum ReadingError {
    /// Error returned when the server fails to read data from the client.
    Reading,

    /// Error returned when the readed packet fails to be decoded
    Decode,
}

#[derive(Debug)]
pub enum ConnectionError {
    /// Error returned when a client fail to connect.
    Client(String),
}

/// Error returned when a packet cannot be decoded from [Packet::decode].
#[derive(Debug)]
pub struct PacketDecodeError;

/// Enum containing all the possible packet types.
#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    // Packet containing data in form of bytes.
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
    // Packet containing data in form of bytes with an identifier which can represent what type of data the packet contains.
    Identified(u32, Vec<u8>),
}

impl Packet {
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Packet::Bytes(data) => {
                result.insert(0, 0);
                for b in data {
                    result.push(*b)
                }
            }
            Packet::String(data) => {
                result.insert(0, 1);
                for b in data.as_bytes() {
                    result.push(*b);
                }
            }
            Packet::I8(data) => {
                result.insert(0, 2);
                result.write_i8(*data);
            }
            Packet::I16(data) => {
                result.insert(0, 3);
                result.write_i16::<LittleEndian>(*data);
            }
            Packet::I32(data) => {
                result.insert(0, 4);
                result.write_i32::<LittleEndian>(*data);
            }
            Packet::I64(data) => {
                result.insert(0, 5);
                result.write_i64::<LittleEndian>(*data);
            }
            Packet::F32(data) => {
                result.insert(0, 6);
                result.write_f32::<LittleEndian>(*data);
            }
            Packet::F64(data) => {
                result.insert(0, 7);
                result.write_f64::<LittleEndian>(*data);
            }
            Packet::U8(data) => {
                result.insert(0, 8);
                result.write_u8(*data);
            }
            Packet::U16(data) => {
                result.insert(0, 9);
                result.write_u16::<LittleEndian>(*data);
            }
            Packet::U32(data) => {
                result.insert(0, 10);
                result.write_u32::<LittleEndian>(*data);
            }
            Packet::U64(data) => {
                result.insert(0, 11);
                result.write_u64::<LittleEndian>(*data);
            }
            Packet::Identified(id, data) => {
                result.insert(0, 12);

                byteorder::LittleEndian::write_u32(&result[1..std::mem::size_of(u32)], *id);

                for b in data {
                    result.push(*b)
                }
            }
        }

        result
    }

    pub fn decode(bytes: Vec<u8>) -> Result<Self, PacketDecodeError> {
        //TODO: Fix this function
        if let Ok(packet) = bincode::deserialize::<Packet>(bytes.as_slice()) {
            Ok(packet)
        } else {
            Err(PacketDecodeError)
        }
    }
}
