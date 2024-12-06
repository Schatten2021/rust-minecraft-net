use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x0F]
pub struct ClearTitles {
    pub reset: bool,
}
impl ClearTitles {
    pub fn new(reset: bool) -> Self {
        Self {reset}
    }
}