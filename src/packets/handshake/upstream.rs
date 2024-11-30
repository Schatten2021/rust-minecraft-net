use crate::{Errors, Packet, PacketReader};
use crate::fields::{encode_string, encode_ushort, encode_var_int};
#[derive(Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: i32,
}
impl Packet for Handshake {
    const ID: i32 = 0x00;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.protocol_version);
        res.append(&mut encode_string(self.server_addr.clone()));
        res.append(&mut encode_ushort(self.server_port));
        res.append(&mut encode_var_int(self.next_state));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        Ok(Self {
            protocol_version: reader.read_var_int()?,
            server_addr: reader.read_string()?,
            server_port: reader.read_ushort(),
            next_state: reader.read_var_int()?,
        })
    }
}
impl Handshake {
    pub fn new(server_addr: String, server_port: u16, next_state: i32) -> Self {
        Self {
            protocol_version: 767, // only supported protocol version rn
            server_addr,
            server_port,
            next_state
        }
    }
}