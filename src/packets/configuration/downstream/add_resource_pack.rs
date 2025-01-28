use minecraft_net_proc::Packet;

Packet!(AddResourcePack, 0x09, {
    uuid: UUID,
    url: String,
    hash: String,
    forced: bool,
    prompt_message: PrefixedOptional<String>,
});