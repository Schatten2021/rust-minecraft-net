use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct Handshake {
    #[Var]
    pub protocol_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    #[Var]
    pub next_state: i32,
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