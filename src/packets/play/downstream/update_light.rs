pub use crate::packets::play::downstream::chunk_data::{BlockLightArray, SkyLightArray};

#[derive(Debug)]
// #[id = 0x2B]
pub struct UpdateLight {
    chunk_x: i32,
    chunk_z: i32,
    // sky_light_mask: TODO: figure out type
    // block_light_mask: TODO: figure out type
    // empty_light_mask: TODO: figure out type
    // empty_block_light_mask: TODO: figure out type
    sky_light_array_count: i32,
    sky_light_arrays: Vec<SkyLightArray>,
    block_light_array_count: i32,
    block_light_arrays: Vec<BlockLightArray>,
}