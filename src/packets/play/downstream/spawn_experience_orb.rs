use minecraft_net_proc::Packet;

Packet!(SpawnExperienceOrb, 0x02, {
    entity_id: VarInt,
    x: Double,
    y: Double,
    z: Double,
    count: Short,
});