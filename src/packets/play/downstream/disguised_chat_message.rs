use minecraft_net_proc::Packet;
use crate::fields::types::TextComponent;

Packet!(DisguisedChatMessage, 0x1E, {
    message: TextComponent,
    chat_type: VarInt,
    sender_name: TextComponent,
    target: PrefixedOptional<TextComponent>,
});