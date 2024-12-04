use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct Explosion {
    x: f64,
    y: f64,
    z: f64,
    strength: f32,
    record_count: i32,
    records: Vec<(u8, u8, u8)>,
    player_motion_x: f32,
    player_motion_y: f32,
    player_motion_z: f32,
    block_interaction: i32,
    small_explosion_particle_id: i32,
    // small_explosion_particle_data: ? TODO: figure out type
    large_explosion_particle_id: i32,
    // large_explosion_particle_data: ? TODO: figure out type
    // explosion_sound: ? TODO: figure out type
}