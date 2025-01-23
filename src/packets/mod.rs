use std::fmt::Debug;
use minecraft_net_proc::Field;
use crate::fields::{encode_bool, encode_long, encode_var_int};
use crate::{Field, PacketReader, Result};
use crate::fields::types::{Identifier, PrefixedArray, PrefixedOptional, VarInt, NBT};

pub mod status;
pub mod handshake;
pub mod login;
pub mod configuration;
pub mod play;

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}
impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Position {
        Position { x, y, z }
    }
}
impl Field for Position {
    fn to_bytes(&self) -> Vec<u8> {
        let val = ((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF);
        encode_long(val)
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Position> {
        let val = reader.read_long();
        let mut x = ((val >> 38) & 0x3FFFFFF) as i32;
        let mut z = ((val >> 12) & 0x3FFFFFF) as i32;
        let mut y = (val & 0xFFF) as i16;
        if x >= 1 << 25 { x -= 1 << 26 }
        if y >= 1 << 11 { y -= 1 << 12 }
        if z >= 1 << 25 { z -= 1 << 26 }
        Ok(Position {x, y, z})
    }
}
pub use slot::*;
mod slot {
    use crate::fields::encode_prefixed_array;
    use crate::fields::types::Double;
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Slot {
        pub item_count: VarInt,
        pub data: Option<SlotData>,
    }
    impl Field for Slot {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = encode_var_int(self.item_count);
            if self.item_count > 0 {
                res.append(&mut self.data.clone().expect("Slot item count bigger than 0 but no data was provided.").to_bytes());
            }
            res
        }

        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            let count = reader.read_var_int()?;
            if count > 0 {
                Ok(Self {
                    item_count: count,
                    data: Some(SlotData::from_reader(reader)?),
                })
            } else {
                Ok(Self {
                    item_count: count,
                    data: None,
                })
            }
        }
    }
    #[derive(Debug, Clone, Field)]
    pub struct SlotData {
        pub item_id: VarInt,
        pub number_of_components_to_add: VarInt,
        pub number_of_components_to_remove: VarInt,
        #[len = "number_of_components_to_add"]
        pub components_to_add: Vec<Component>,
        #[len = "number_of_components_to_remove"]
        pub components_to_remove: Vec<VarInt>,
    }
    #[derive(Debug, Clone)]
    pub enum Component {
        CustomData(NBT),
        MaxStackSize(VarInt),
        MaxDamage(VarInt),
        Damage(VarInt),
        Unbreakable(bool),
        CustomName(String),
        ItemName(String),
        ItemModel(Identifier),
        Lore(PrefixedArray<String>),
        Rarity(VarInt),
        Enchantments(OptionalTooltip<PrefixedArray<Enchantment>>),
        CanPlaceOn(BlockInteractions),
        CanBreak(BlockInteractions),
        AttributeModifiers(OptionalTooltip<PrefixedArray<AttributeModifier>>)
    }
    impl Field for Component {
        fn to_bytes(&self) -> Vec<u8> {
            todo!()
        }

        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            todo!()
        }
    }
    #[derive(Debug, Clone)]
    pub struct OptionalTooltip<T: Debug + Clone> {
        pub value: T,
        pub show_tooltip: bool,
    }
    impl<T: Debug + Clone + Field> Field for OptionalTooltip<T> {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = self.value.to_bytes();
            res.append(&mut encode_bool(self.show_tooltip));
            res
        }
        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            Ok(Self {
                value: T::from_reader(reader)?,
                show_tooltip: reader.read_bool()?,
            })
        }
    }
    impl<T: Debug + Clone + Field> Field for OptionalTooltip<PrefixedArray<T>> {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = encode_prefixed_array(&self.value);
            res.append(&mut encode_bool(self.show_tooltip));
            res
        }
        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            Ok(Self {
                value: reader.read_prefixed_array()?,
                show_tooltip: reader.read_bool()?,
            })
        }
    }
    #[derive(Debug, Clone, Field)]
    pub struct Enchantment {
        pub type_id: VarInt,
        pub level: VarInt,
    }
    #[derive(Clone, Debug, Field)]
    pub struct BlockInteractions {
        pub block_predicates: PrefixedArray<BlockPredicate>,
        pub show_in_tooltip: bool,
    }
    #[derive(Clone, Debug, Field)]
    pub struct AttributeModifier {
        pub attribute_id: VarInt,
        pub modifier_id: Identifier,
        pub value: Double,
        pub operation: VarInt,
        pub slot: VarInt,
    }
}
pub use block_predicate::*;
mod block_predicate {
    use crate::fields::encode_string;
    use super::*;
    #[derive(Debug, Clone, Field)]
    pub struct BlockPredicate {
        pub blocks: PrefixedOptional<IDSet>,
        pub properties: PrefixedOptional<PrefixedArray<Property>>,
        pub nbt: PrefixedOptional<NBT>,
    }
    #[derive(Debug, Clone)]
    pub struct IDSet {
        pub r#type: VarInt,
        pub tag_name: Option<String>,
        pub ids: Option<Vec<VarInt>>
    }
    impl Field for IDSet {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = encode_var_int(self.r#type);
            if self.r#type == 0 {
                res.append(&mut encode_string(self.tag_name.clone().expect("Tag name not provided")));
            } else {
                res.append(&mut self.ids.clone().expect("type not 0 but no ids given")
                    .iter().flat_map(|id| encode_var_int(*id)).collect());
            }
            res
        }
        fn from_reader(reader: &mut PacketReader) -> Result<Self> {
            let r#type = reader.read_var_int()?;
            if r#type == 0 {
                Ok(Self {
                    r#type,
                    tag_name: Some(reader.read_identifier()?),
                    ids: None
                })
            } else {
                let len = (r#type - 1) as usize;
                let mut vec = Vec::with_capacity(len);
                for _ in 0..len {
                    vec.push(reader.read_var_int()?);
                }
                Ok(Self {
                    r#type,
                    tag_name: None,
                    ids: Some(vec)
                })
            }
        }
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
}