use crate::fields::general::Position;
use minecraft_net_proc::Packet;
Packet!(BlockAction, 0x08, {
    location: Position,
    action_id: UByte,
    action_parameter: UByte,
    block_type: VarInt,
});