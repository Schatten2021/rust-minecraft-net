use minecraft_net_proc::{Field, Packet};

Field!(Property, {
    name: String,
    value: String,
    signature: PrefixedOptional<String>,
});
Packet!(LoginSuccess, 0x02, {
    uuid: UUID,
    username: String,
    properties: PrefixedArray<Property>,
});