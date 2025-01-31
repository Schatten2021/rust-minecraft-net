use crate::fields::general::Position;
use minecraft_net_proc::{Field, Packet};

Packet!(Login, 0x2C, {
    entity_id: Int,
    is_hardcore: bool,
    dimension_names: PrefixedArray<String>,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: Identifier,
    hashed_seed: Long,
    game_mode: UByte,
    previous_game_mode: Byte,
    is_debug: bool,
    is_flat: bool,
    death_information: PrefixedOptional<DeathInformation>,
    portal_cooldown: VarInt,
    sea_level: VarInt,
    enforces_secure_chat: bool,
});
Field!(DeathInformation, {
    dimension_name: Identifier,
    location: Position,
});