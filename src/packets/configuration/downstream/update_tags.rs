use minecraft_net_proc::{Field, Field_old, Packet, Packet_old};
use crate::fields::types::PrefixedArray;

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