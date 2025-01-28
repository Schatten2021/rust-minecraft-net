use minecraft_net_proc::Packet;

Packet!(PlayerInfoRemoved, 0x3F, {
    uuids: PrefixedArray<UUID>,
});