use minecraft_net_proc::Packet;
use crate::fields::nbt::TextComponent;

Packet!(AddResourcePack, 0x09, {
    uuid: UUID,
    url: String,
    hash: String,
    forced: bool,
    prompt_message: PrefixedOptional<TextComponent>,
});