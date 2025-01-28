use minecraft_net_proc::{Packet, VarIntEnum};

VarIntEnum!(ChatSuggestionAction, {
    Add, Remove, Set,
});
Packet!(ChatSuggestion, 0x18, {
    action: ChatSuggestionAction,
    entries: PrefixedArray<String>,
});