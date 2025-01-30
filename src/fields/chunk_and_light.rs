use minecraft_net_proc::Field;
use crate::fields::general::BitSet;

Field!(ChunkData, {
    height_maps: NBT,
    data: PrefixedArray<Byte>,
    block_entities: PrefixedArray<BlockEntity>,
});
Field!(BlockEntity, {
    packed_xz: UByte,
    y: Short,
    r#type: VarInt,
    data: NBT,
});
Field!(LightData, {
    sky_light_mask: BitSet,
    block_light_mask: BitSet,
    empty_sky_light_mask: BitSet,
    empty_block_light_mask: BitSet,
    sky_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
    block_light_arrays: PrefixedArray<PrefixedArray<Byte>>,
});
