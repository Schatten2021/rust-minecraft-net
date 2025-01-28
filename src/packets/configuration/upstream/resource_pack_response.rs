use minecraft_net_proc::Packet;
Packet!(ResourcePackResponse, 0x06, {
    uuid: UUID,
    result: VarInt,
});