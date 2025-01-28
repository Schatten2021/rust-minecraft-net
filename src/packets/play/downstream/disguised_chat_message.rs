use minecraft_net_proc::Packet;

Packet!(DisguisedChatMessage, 0x1E, {
    message: String,
    chat_type: VarInt,
    sender_name: String,
    target: PrefixedOptional<String>,
});