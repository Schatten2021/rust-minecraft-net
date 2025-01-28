use minecraft_net_proc::{Field, Packet};

Field!(Entry, {
    entry_id: String,
    data: PrefixedOptional<NBT>
});
Packet!(RegistryData, 0x07, {
    registry_id: String,
    entries: PrefixedArray<Entry>,
});