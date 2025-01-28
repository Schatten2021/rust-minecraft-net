use minecraft_net_proc::{Field, Packet};

Field!(Matches, {
    r#match: String,
    tooltip: PrefixedOptional<String>,
});
Packet!(CommandSuggestionsResponse, 0x10, {
    id: VarInt,
    start: VarInt,
    length: VarInt,
    matches: PrefixedArray<Matches>,
});