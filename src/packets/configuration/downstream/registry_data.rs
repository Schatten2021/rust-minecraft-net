use crate::{Errors, Packet, PacketReader, Result};
use crate::fields::{encode_bool, encode_string, encode_var_int};
//TODO: implement Packet
#[derive(Debug)]
pub struct Entry {
    pub entry_id: String,
    pub has_data: bool,
    pub data: Option<crab_nbt::Nbt>,
}
impl Entry {
    pub fn new(id: String, data: Option<crab_nbt::Nbt>) -> Self {
        Self {
            entry_id: id,
            has_data: data.is_none(),
            data,
        }
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let id = reader.read_string()?;
        let has_data = reader.read_bool()?;
        if has_data {
            let nbt = crab_nbt::Nbt::read_unnamed(reader).map_err(|e| Errors::NbtError(e))?;
            Ok(Self {
                entry_id: id,
                has_data,
                data: Some(nbt)
            })
        } else {
            Ok(Self {
                entry_id: id,
                has_data,
                data: None
            })
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.entry_id.clone());
        res.append(&mut encode_bool(self.has_data));
        if self.has_data {
            let mut data = self.data.clone().unwrap().write_unnamed().to_vec();
            res.append(&mut data);
        }
        res
    }
}

#[derive(Debug)]
pub struct RegistryData {
    pub registry_id: String,
    pub entry_count: i32,
    pub entries: Vec<Entry>
}
impl Packet for RegistryData {
    const ID: i32 = 0x07;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.registry_id.clone());
        res.append(&mut encode_var_int(self.entry_count));
        res.append(&mut self.entries.iter().flat_map(|e| e.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let id = reader.read_string()?;
        let entry_count = reader.read_var_int()? as usize;
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            entries.push(Entry::from_reader(reader)?)
        }
        Ok(Self {
            registry_id: id,
            entry_count: entry_count as i32,
            entries,
        })
    } 
}
impl RegistryData {
    pub fn new(id: String, entries: Vec<Entry>) -> Self {
        Self {
            registry_id: id,
            entry_count: entries.len() as i32,
            entries,
        }
    }
}