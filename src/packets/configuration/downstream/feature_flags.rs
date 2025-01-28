use minecraft_net_proc::Packet;

Packet!(FeatureFlags, 0x0C, {
    feature_flags: PrefixedArray<String>
});