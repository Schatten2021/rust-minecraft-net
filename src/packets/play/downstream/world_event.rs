use crate::packets::Position;
use minecraft_net_proc::Packet;

Packet!(WorldEvent, 0x29, {
    event: Int,
    location: Position,
    data: Int,
    disable_relative_volume: bool
});