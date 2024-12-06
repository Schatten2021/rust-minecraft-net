use crate::fields::{encode_identifier, encode_var_int};
use crate::{Packet, PacketReader, Result};
//TODO: implement Packet
#[derive(Debug)]
pub struct Tag {
    pub tag_name: String,
    pub count: i32,
    pub entries: Vec<i32>
}
impl Tag {
    pub fn new(name: String, entries: Vec<i32>) -> Self {
        Self {
            tag_name: name,
            count: entries.len() as i32,
            entries,
        }
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let tag_name = reader.read_identifier()?;
        let count = reader.read_var_int()?;
        let mut entries = Vec::with_capacity(count as usize);
        for _ in 0..count {
            entries.push(reader.read_var_int()?)
        }
        Ok(Self {
            tag_name,
            count,
            entries
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_identifier(self.tag_name.clone());
        res.append(&mut encode_var_int(self.count));
        res.append(&mut self.entries.iter().flat_map(|e| encode_var_int(*e)).collect());
        res
    }
}
#[derive(Debug)]
pub struct Tags {
    pub registry: String,
    pub tags: Vec<Tag>
}
impl Tags {
    pub fn new(registry: String, tags: Vec<Tag>) -> Self {
        Self {registry, tags}
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let registry = reader.read_string()?;
        let length = reader.read_var_int()?;
        let mut tags = Vec::with_capacity(length as usize);
        for _ in 0..length {
            tags.push(Tag::from_reader(reader)?)
        }
        Ok(Self {
            registry, tags,
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_identifier(self.registry.clone());
        res.append(&mut encode_var_int(self.tags.len() as i32));
        res.append(&mut self.tags.iter().flat_map(|t| t.to_bytes()).collect());
        res
    }
}
#[derive(Debug)]
pub struct UpdateTags {
    pub len: i32,
    pub arr: Vec<Tags>
}
impl Packet for UpdateTags {
    const ID: i32 = 0x0D;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.len);
        res.append(&mut self.arr.iter().flat_map(|t| t.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let len = reader.read_var_int()?;
        let mut arr = Vec::with_capacity(len as usize);
        for _ in 0..len {
            arr.push(Tags::from_reader(reader)?)
        }
        Ok(Self {len, arr})
    }
}
impl UpdateTags {
    pub fn new(tags: Vec<Tags>) -> Self {
        Self {
            len: tags.len() as i32,
            arr: tags,
        }
    }
}