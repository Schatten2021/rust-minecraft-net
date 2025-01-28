use minecraft_net_proc::Packet;

Packet!(InitializeWorldBorder, 0x26, {
    x: Double,
    y: Double,
    old_diameter: Double,
    new_diameter: Double,
    speed: VarInt,
    portal_teleport_boundary: VarInt,
    warning_blocks: VarInt,
    warning_time: VarInt,
});