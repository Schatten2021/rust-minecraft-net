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
    fn to_bytes(&self) -> Vec<u8>;
    fn from_reader(reader: &mut PacketReader) -> Result<Self>;
}
pub trait Stream: Read + Write {}
impl<T> Stream for T where T: Read + Write {}
#[macro_export] macro_rules! join {
    () => { vec![] };
    ( $( $vec:expr ),* ) => {
        vec![$( $vec ),*].iter().flatten().cloned().collect()
    };
}