use minecraft_net_proc::Packet_old;
use crate::fields::general::Position;

#[derive(Debug, Packet_old)]
#[id = 0x2C]
pub struct Login {
    entity_id: i32,
    is_hardcore: bool,
    dimension_count: i32,
    #[len = "dimension_count"]
    dimension_names: Vec<String>,
    max_players: i32,
    view_distance: i32,
    simulation_distance: i32,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: i32,
    dimension_name: String,
    hashed_seed: i64,
    game_mode: u8,
    previous_game_mode: i8,
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool,
    #[when = "has_death_location"]
    death_dimension_name: Option<String>,
    #[when = "has_death_location"]
    death_location: Option<Position>,
    portal_cooldown: i32,
    enforces_secure_chat: bool,
}