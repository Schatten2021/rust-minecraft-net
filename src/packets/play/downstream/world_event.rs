use crate::packets::Position;
use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x28]
pub struct WorldEvent {
    #[Const]
    pub event: i32,
    pub location: Position,
    #[Const]
    pub data: i32,
    pub disable_relative_volume: bool,
}
impl WorldEvent {
    pub fn new(event: i32, location: Position, data: i32, disable_relative_volume: bool) -> Self {
        Self {event, location, data, disable_relative_volume}
    }
}