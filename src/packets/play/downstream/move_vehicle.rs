use minecraft_net_proc::Packet;

Packet!(MoveVehicle, 0x33, {
    x: Double,
    y: Double,
    z: Double,
    yaw: Float,
    pitch: Float,
});