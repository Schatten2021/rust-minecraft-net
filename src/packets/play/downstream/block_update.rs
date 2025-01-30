use crate::fields::general::Position;
use minecraft_net_proc::Packet;

Packet!(BlockUpdate, 0x09, {
    location: Position,
    block_id: VarInt,
});