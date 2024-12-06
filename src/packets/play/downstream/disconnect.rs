use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x1D]
pub struct Disconnect {
    pub reason: String,
}
impl Disconnect {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}