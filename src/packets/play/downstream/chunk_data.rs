#[derive(Debug)]
pub struct BlockEntity {
    packed_xz: u8,
    y: i16,
    r#type: i32,
    data: crab_nbt::Nbt,
}

#[derive(Debug)]
pub struct SkyLightArray {
    length: i32,
    sky_light_array: Vec<u8>
}
#[derive(Debug)]
pub struct BlockLightArray {
    length: i32,
    block_light_array: Vec<u8>
}

#[derive(Debug)]
pub struct ChunkDataWithLight {
    chunk_x: i32,
    chunk_z: i32,
    height_maps: crab_nbt::Nbt,
    size: i32,
    data: Vec<u8>,
    number_of_block_entities: i32,
    block_entities: Vec<BlockEntity>,
    // sky_light_mask: ? TODO: implement BitSet
    // block_light_mask: ? TODO: implement BitSet
    // empty_sky_light_mask: ? TODO: implement BitSet
    // empty_block_light_mask: ? TODO: implement BitSet
    sky_light_array_count: i32,
    sky_light_arrays: Vec<SkyLightArray>,
    block_light_array_count: i32,
    block_light_arrays: Vec<BlockLightArray>,
}