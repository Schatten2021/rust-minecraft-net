use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x03]
pub struct LoginAcknowledged {}
impl LoginAcknowledged {
    pub fn new() -> Self {Self {}}
}