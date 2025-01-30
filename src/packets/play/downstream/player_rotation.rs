use minecraft_net_proc::Packet;

Packet!(PlayerRotation, 0x43, {
    yaw: Float,
    pitch: Float,
});