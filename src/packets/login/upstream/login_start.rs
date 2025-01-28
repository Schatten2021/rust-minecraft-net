use minecraft_net_proc::Packet;

Packet!(LoginStart, 0x00, {
    name: String,
    uuid: UUID,
});