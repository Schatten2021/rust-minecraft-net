use minecraft_net_proc::Packet;

Packet!(Particle, 0x2A, {
    long_distance: bool,
    x: Double,
    y: Double,
    z: Double,
    offset_x: Float,
    offset_y: Float,
    offset_z: Float,
    max_speed: Float,
    particle_count: Int,
    data: crate::packets::Particle,
});