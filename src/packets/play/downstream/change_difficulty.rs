use minecraft_net_proc::Packet;
Packet!(ChangeDifficulty, 0x0B, {
    difficulty: UByte,
    difficulty_locked: bool,
});