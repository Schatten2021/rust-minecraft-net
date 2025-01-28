use minecraft_net_proc::Packet;

Packet!(OpenHorseScreen, 0x24, {
    window_id: UByte,
    slot_count: VarInt,
    entity_id: VarInt,
});