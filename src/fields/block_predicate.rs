use crate::fields::types::*;
use crate::fields::types::{PrefixedOptional, NBT};
use crate::fields::{encode_bool, encode_string};
use crate::fields::general::IDSet;
use crate::{Field, PacketReader, Result};
use minecraft_net_proc::Field_old;

#[derive(Debug, Clone, Field_old)]
pub struct BlockPredicate {
    pub blocks: PrefixedOptional<IDSet>,
    pub properties: PrefixedOptional<PrefixedArray<Property>>,
    pub nbt: PrefixedOptional<NBT>,
}
#[derive(Debug, Clone)]
pub struct Property {
    name: String,
    is_exact_match: bool,
    exact_value: Option<String>,
    min_value: Option<String>,
    max_value: Option<String>,
}
impl Field for Property {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.name.clone());
        res.append(&mut encode_bool(self.is_exact_match));
        if self.is_exact_match {
            res.append(&mut encode_string(self.exact_value.clone().expect("Property value not provided")));
        } else {
            res.append(&mut encode_string(self.min_value.clone().expect("Property min value not provided")));
            res.append(&mut encode_string(self.max_value.clone().expect("Property max value not provided")));
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let name = reader.read_string()?;
        let is_exact_match = reader.read_bool()?;
        if is_exact_match {
            Ok(Self {
                name, is_exact_match,
                exact_value: Some(reader.read_string()?),
                min_value: None,
                max_value: None,
            })
        } else {
            Ok(Self {
                name, is_exact_match,
                exact_value: None,
                min_value: Some(reader.read_string()?),
                max_value: Some(reader.read_string()?),
            })
        }
    }
}