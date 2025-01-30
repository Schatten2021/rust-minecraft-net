use minecraft_net_proc::Packet;
use crate::fields::chunk_and_light::LightData;

Packet!(UpdatLight, 0x2B, {
    chunk_x: VarInt,
    chunk_y: VarInt,
    data: LightData
});