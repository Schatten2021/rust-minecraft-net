use crate::packets::Position;
use minecraft_net_proc::Packet;
use crate::fields::types::Int;

#[derive(Debug, Packet)]
#[id = 0x29]
pub struct WorldEvent {
    pub event: Int,
    pub location: Position,
    pub data: Int,
    pub disable_relative_volume: bool,
}
impl WorldEvent {
    pub fn new(event: i32, location: Position, data: i32, disable_relative_volume: bool) -> Self {
        Self {event, location, data, disable_relative_volume}
    }
}