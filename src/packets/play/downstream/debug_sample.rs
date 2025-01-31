use minecraft_net_proc::{Packet, VarIntEnum};

Packet!(DebugSample, 0x1B, {
    sample: PrefixedArray<Long>,
    sample_type: SampleType,
});
VarIntEnum!(SampleType, {
    TickTime,
});