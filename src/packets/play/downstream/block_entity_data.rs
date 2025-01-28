use crate::packets::Position;
use minecraft_net_proc::Packet;

Packet!(BlockEntityData, 0x07, {
    location: Position,
    r#type: VarInt,
    data: NBT,
});