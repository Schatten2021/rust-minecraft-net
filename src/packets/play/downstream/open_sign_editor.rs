use minecraft_net_proc::Packet;
use crate::packets::Position;

#[derive(Packet)]
#[id = 0x36]
pub struct OpenSignEditor {
    pub location: Position,
    pub is_front_text: bool,
}
impl OpenSignEditor {
    pub fn new(location: Position, is_front_text: bool) -> Self {
        Self {location, is_front_text}
    }
}