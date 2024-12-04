use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_bool, encode_string, encode_var_int};

#[derive(Debug)]
pub struct Matches {
    pub r#match: String,
    pub has_tooltip: bool,
    pub tooltip: Option<String>
}
impl Matches {
    pub fn new(r#match: String, tooltip: Option<String>) -> Self {
        Self {
            r#match,
            has_tooltip: tooltip.is_some(),
            tooltip,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.r#match.clone());
        res.append(&mut encode_bool(self.has_tooltip));
        if self.has_tooltip {
            res.append(&mut encode_string(self.tooltip.clone().unwrap()));
        }
        res
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Matches> {
        let r#match = reader.read_string()?;
        let has_tooltip = reader.read_bool()?;
        if has_tooltip {
            Ok(Self {
                r#match,
                has_tooltip,
                tooltip: Some(reader.read_string()?)
            })
        } else {
            Ok(Self {
                r#match,
                has_tooltip,
                tooltip: None,
            })
        }
    }
}

#[derive(Debug)]
pub struct CommandSuggestionResponse {
    pub id: i32,
    pub start: i32,
    pub length: i32,
    pub count: i32,
    pub matches: Vec<Matches>
}
impl Packet for CommandSuggestionResponse {
    const ID: i32 = 0x10;
    fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.id),
            encode_var_int(self.start),
            encode_var_int(self.length),
            encode_var_int(self.count),
            self.matches.iter().flat_map(|m| m.to_bytes()).collect()
        ].iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let id = reader.read_var_int()?;
        let start = reader.read_var_int()?;
        let length = reader.read_var_int()?;
        let count = reader.read_var_int()?;
        let mut matches = Vec::with_capacity(count as usize);
        for _ in 0..count {
            matches.push(Matches::from_reader(reader)?);
        }
        Ok(Self {id, start, length, count, matches})
    }
}
impl CommandSuggestionResponse {
    pub fn new(id: i32, start: i32, length: i32, matches: Vec<Matches>) -> Self {
        Self {id, start, length, count: matches.len() as i32, matches}
    }
}