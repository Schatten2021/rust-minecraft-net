use minecraft_net_proc::Packet;
Packet!(ClientInformation, 0x00, {
    locale: String,
    view_distance: Byte,
    chat_mode: VarInt,
    chat_colors: bool,
    displayed_skin_parts: UByte,
    main_hand: VarInt,
    enable_text_filtering: bool,
    allow_server_listings: bool,
});