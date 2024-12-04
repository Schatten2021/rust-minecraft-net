use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_string, encode_var_int};

#[derive(Debug)]
pub struct ChatSuggestion {
    action: i32,
    count: i32,
    entries: Vec<String>,
}
impl Packet for ChatSuggestion {
    const ID: i32 = 0x18;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.action),
            encode_var_int(self.count),
            self.entries.iter().flat_map(|e| encode_string(e.clone())).collect(),
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let action = reader.read_var_int()?;
        let count = reader.read_var_int()?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            entries.push(reader.read_string()?);
        }
        Ok(Self { action, count, entries })
    }
}
impl ChatSuggestion {
    pub fn new(action: i32, entries: Vec<String>) -> Self {
        Self {
            action,
            count: entries.len() as i32,
            entries,
        }
    }
}