pub mod fields;
mod errors;
mod net;
pub mod packets;

use std::io::{Read, Write};
use crate::errors::Result;
pub use crate::errors::Errors;
pub use crate::fields::PacketReader;
pub use crate::net::{send_packet, receive_packet, receive_unknown_packet, UnknownPacket, EncryptedTcp};

pub trait Packet: Sized {
    const ID: i32;
    /// Serializes the packet to a Vec\<u8> in accordance with the minecraft protocol so that it can be sent via the network.
    ///
    /// # Arguments
    /// returns: Vec\<u8>
    fn to_bytes(&self) -> Vec<u8>;
    /// Allows the reading of packets from a PacketReader.
    ///
    /// # Arguments
    ///
    /// * `reader`: The reader the packet will be read from.
    ///
    /// returns: Result<Self, Errors>
    fn from_reader(reader: &mut PacketReader) -> Result<Self>;
}
pub trait Field: Sized {
    /// Serializes the field into an array of bytes in accordance with the minecraft protocol.
    ///
    /// # Arguments
    /// returns: Vec\<u8>
    fn to_bytes(&self) -> Vec<u8>;
    /// Allows a field to be read from a PacketReader, which is a mostly internal struct.
    ///
    /// # Arguments
    ///
    /// * `reader`: The reader which field packets will be read from
    ///
    /// returns: Result<Self, Errors>
    fn from_reader(reader: &mut PacketReader) -> Result<Self>;
}
impl<T> Field for T where T: Packet {
    fn to_bytes(&self) -> Vec<u8> {self.to_bytes()}
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {Self::from_reader(reader)}
}
impl Field for crab_nbt::Nbt {
    fn to_bytes(&self) -> Vec<u8> {
        self.write_unnamed().to_vec()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Self::read(reader).map_err(|e| Errors::NbtError(e))
    }
}

pub trait Stream: Read + Write {}
impl<T> Stream for T where T: Read + Write {}
#[macro_export] macro_rules! join {
    () => { vec![] };
    ( $( $vec:expr ),* ) => {
        vec![$( $vec ),*].iter().flatten().cloned().collect()
    };
}