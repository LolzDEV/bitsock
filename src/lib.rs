use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
#[derive(Debug)]
pub enum Packet {
    Invalid,
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
    /// Encode the packet into bytes.
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Packet::Bytes(data) => {
                result.insert(0, 1);
                for _ in 0..(std::mem::size_of::<u8>() * data.len()) {
                    result.push(0);
                }
                for b in data {
                    result.push(*b)
                }
            }
            Packet::String(data) => {
                result.insert(0, 2);
                for _ in 0..std::mem::size_of::<String>() {
                    result.push(0);
                }
                for b in data.as_bytes() {
                    result.push(*b);
                }
            }
            Packet::I8(data) => {
                result.insert(0, 3);
                for _ in 0..std::mem::size_of::<i8>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_i8(*data);
            }
            Packet::I16(data) => {
                result.insert(0, 4);
                for _ in 0..std::mem::size_of::<i16>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_i16::<LittleEndian>(*data);
            }
            Packet::I32(data) => {
                result.insert(0, 5);
                for _ in 0..std::mem::size_of::<i32>() {
                    result.push(0);
                }
                let _ = &result[1..]
                    .as_mut()
                    .write_i32::<LittleEndian>(*data)
                    .unwrap();
            }
            Packet::I64(data) => {
                result.insert(0, 6);
                for _ in 0..std::mem::size_of::<i64>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_i64::<LittleEndian>(*data);
            }
            Packet::F32(data) => {
                result.insert(0, 7);
                for _ in 0..std::mem::size_of::<f32>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_f32::<LittleEndian>(*data);
            }
            Packet::F64(data) => {
                result.insert(0, 8);
                for _ in 0..std::mem::size_of::<f64>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_f64::<LittleEndian>(*data);
            }
            Packet::U8(data) => {
                result.insert(0, 9);
                for _ in 0..std::mem::size_of::<u8>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_u8(*data);
            }
            Packet::U16(data) => {
                result.insert(0, 10);
                for _ in 0..std::mem::size_of::<u16>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_u16::<LittleEndian>(*data);
            }
            Packet::U32(data) => {
                result.insert(0, 11);
                for _ in 0..std::mem::size_of::<u32>() {
                    result.push(0);
                }
                let _: &Result<(), std::io::Error> =
                    &result[1..].as_mut().write_u32::<LittleEndian>(*data);
            }
            Packet::U64(data) => {
                result.insert(0, 12);
                for _ in 0..std::mem::size_of::<u64>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_u64::<LittleEndian>(*data);
            }
            Packet::Identified(id, data) => {
                result.insert(0, 13);
                for _ in 0..std::mem::size_of::<u32>() {
                    result.push(0);
                }
                let _ = &result[1..].as_mut().write_u32::<LittleEndian>(*id);

                for b in data {
                    result.push(*b)
                }
            }
            Packet::Invalid => (),
        }

        result
    }

    /// Returns a [Packet] from a [Vec] of bytes.
    pub fn decode(bytes: Vec<u8>) -> Result<Self, PacketDecodeError> {
        if let Some(b) = bytes.first() {
            match b {
                1 => Ok(Packet::Bytes(bytes[1..].to_vec())),
                2 => Ok(Packet::String(
                    if let Ok(s) = String::from_utf8(bytes[1..].to_vec()) {
                        s
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                3 => Ok(Packet::I8(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_i8() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                4 => Ok(Packet::I16(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_i16::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                5 => Ok(Packet::I32(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_i32::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                6 => Ok(Packet::I64(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_i64::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                7 => Ok(Packet::F32(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_f32::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                8 => Ok(Packet::F64(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_f64::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                9 => Ok(Packet::U8(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_u8() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                10 => Ok(Packet::U16(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_u16::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                11 => Ok(Packet::U32(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_u32::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                12 => Ok(Packet::U64(
                    if let Ok(n) = Cursor::new(bytes[1..].to_vec()).read_u64::<LittleEndian>() {
                        n
                    } else {
                        return Err(PacketDecodeError);
                    },
                )),
                13 => Ok(Packet::Identified(
                    if let Ok(id) = Cursor::new(bytes[1..].to_vec()).read_u32::<LittleEndian>() {
                        id
                    } else {
                        return Err(PacketDecodeError);
                    },
                    bytes[std::mem::size_of::<u32>()..].to_vec(),
                )),
                _ => Ok(Packet::Invalid),
            }
        } else {
            Err(PacketDecodeError)
        }
    }
}

/// Enum used to specify if the log is generated by a physical client or a physical server.
pub enum LogStage {
    SERVER,
    CLIENT,
}

/// Enum used to specify the `level` of a log.
#[derive(Debug)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
}
