use minecraft_net_proc::{Field, Packet};

Field!(Entry, {
    entry_id: Identifier,
    data: PrefixedOptional<NBT>
});
Packet!(RegistryData, 0x07, {
    registry_id: Identifier,
    entries: PrefixedArray<Entry>,
});