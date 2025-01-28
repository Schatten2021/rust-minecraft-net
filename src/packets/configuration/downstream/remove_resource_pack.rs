use minecraft_net_proc::Packet;

Packet!(RemoveResourcePack, 0x08, {
    uuid: PrefixedOptional<UUID>
});