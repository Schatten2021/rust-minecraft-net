use minecraft_net_proc::Packet;
Packet!(ClearTitles, 0x0F, {
    reset: bool,
});