use minecraft_net_proc::{Field, Packet};

Field!(Statistic, {
    category_id: VarInt,
    statistics_id: VarInt,
    value: VarInt,
});
Packet!(AwardStatistics, 0x04, {
    statistics: PrefixedArray<Statistic>
});