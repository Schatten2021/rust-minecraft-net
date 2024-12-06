use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x02]
pub struct Disconnect {
    pub reason: String,
}
impl Disconnect {
    pub fn new(reason: String) -> Self {
        Self {reason}
    }
}