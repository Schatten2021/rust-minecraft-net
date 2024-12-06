use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x03]
pub struct FinishConfiguration {}
impl FinishConfiguration {
    pub fn new() -> Self {Self{} }
}