use minecraft_net_proc::Packet_old;
use crate::fields::types::VarInt;

#[derive(Debug, Packet_old)]
#[id = 0x00]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: VarInt,
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