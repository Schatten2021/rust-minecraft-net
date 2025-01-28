use minecraft_net_proc::Packet;

Packet!(PlayerAbilities, 0x3A, {
    flags: Byte,
    flying_speed: Float,
    fov_modifier: Float,
});