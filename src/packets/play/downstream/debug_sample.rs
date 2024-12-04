use crate::{Packet, PacketReader, Result};

#[derive(Debug)]
pub struct DebugSample {
    sample_length: i32,
    //TODO: figure out what this type is
    //sample: Vec<?>
    sample_type: i32,
}