use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x06]
pub struct ResetChat {}
impl ResetChat {
    pub fn new() -> Self {Self {}}
}