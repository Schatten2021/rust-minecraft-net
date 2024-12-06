use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x12]
pub struct CloseContainer {
    pub window_id: u8
}
impl CloseContainer {
    pub fn new(window_id: u8) -> Self {
        Self {window_id}
    }
}