use minecraft_net_proc::{Packet, VarIntEnum};
VarIntEnum!(ChatMode, {
    Enabled, CommandsOnly, Hidden,
});
VarIntEnum!(ParticleStatus, {
    All, Decreased, Minimal
});
Packet!(ClientInformation, 0x00, {
    locale: String,
    view_distance: Byte,
    chat_mode: ChatMode,
    chat_colors: bool,
    displayed_skin_parts: UByte,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listings: bool,
    particle_status: ParticleStatus,
});