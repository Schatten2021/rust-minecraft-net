use crate::packets::Position;
use minecraft_net_proc::Packet;
Packet!(SetBlockDestroyStage, 0x06, {
    entity_id: VarInt,
    location: Position,
    destroy_stage: Int,
});