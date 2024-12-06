#[derive(Debug)]
pub struct Particle {
    long_distance: bool,
    x: f64,
    y: f64,
    z: f64,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
    max_speed: f32,
    particle_count: i32,
    particle_id: i32,
    // data: ? TODO: figure out type
}