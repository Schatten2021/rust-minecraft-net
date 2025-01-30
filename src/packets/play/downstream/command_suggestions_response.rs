use minecraft_net_proc::{Field, Packet};
use crate::fields::types::TextComponent;

Field!(Matches, {
    r#match: String,
    tooltip: PrefixedOptional<TextComponent>,
});
Packet!(CommandSuggestionsResponse, 0x10, {
    id: VarInt,
    start: VarInt,
    length: VarInt,
    matches: PrefixedArray<Matches>,
});