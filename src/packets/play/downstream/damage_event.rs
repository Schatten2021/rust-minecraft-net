use crate::fields::types::Position;
use minecraft_net_proc::{Field, Packet};
Field!(SourcePosition, {
    x: Double,
    y: Double,
    z: Double,
});
Packet!(DamageEvent, 0x1A, {
    entity_id: VarInt,
    source_type_id: VarInt,
    source_cause_id: VarInt,
    source_direct_id: VarInt,
    source_position: PrefixedOptional<Position>,
});