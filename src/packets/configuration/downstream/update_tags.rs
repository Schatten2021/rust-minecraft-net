use minecraft_net_proc::{Field, Packet};

Field!(Tag, {
    tag_name: String,
    entries: PrefixedArray<VarInt>
});
Field!(Tags, {
    registry: String,
    tags: PrefixedArray<Tag>,
});
Packet!(UpdateTags, 0x0D, {
    data: PrefixedArray<Tags>,
});