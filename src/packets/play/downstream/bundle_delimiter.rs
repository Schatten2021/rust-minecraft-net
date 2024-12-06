use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x00]
pub struct BundleDelimiter {}

impl BundleDelimiter {
    pub fn new() -> Self {Self {}}
}