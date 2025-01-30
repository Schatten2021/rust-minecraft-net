use crate::fields::types::{Byte, Double, Float, Int, Long, Short, UShort};
use crate::fields::{encode_double, encode_float, encode_int, encode_long, encode_short, encode_ushort};
use crate::PacketReader;
use crate::{Errors, Field, Result};

macro_rules! prefix {
    ($id:expr, $data:expr) => {vec![$id].into_iter().chain($data).collect::<Vec<u8>>()};
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    TagEnd,
    TagByte(Byte),
    TagShort(Short),
    TagInt(Int),
    TagLong(Long),
    TagFloat(Float),
    TagDouble(Double),
    TagByteArray(Vec<i8>),
    TagString(String),
    TagList(Vec<Tag>),
    TagCompound(Vec<NBT>),
    TagIntArray(Vec<Int>),
    TagLongArray(Vec<Long>),
}
impl Tag {
    fn from_reader_with_type(reader: &mut PacketReader, id: u8) -> Result<Self> {
        Ok(match id {
            0 => Self::TagEnd,
            1 => Self::TagByte(reader.read_byte()),
            2 => Self::TagShort(reader.read_short()),
            3 => Self::TagInt(reader.read_int()),
            4 => Self::TagLong(reader.read_long()),
            5 => Self::TagFloat(reader.read_float()),
            6 => Self::TagDouble(reader.read_double()),
            7 => {
                let len = reader.read_int();
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(reader.read_byte());
                }
                Self::TagByteArray(arr)
            },
            8 => {
                let len = reader.read_ushort();
                let string = cesu8::from_java_cesu8(&*reader.read_byte_array(len as usize))?.to_string();
                Self::TagString(string) 
            },
            9 => {
                let type_id = reader.read_ubyte();
                let len = reader.read_int();
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(Self::from_reader_with_type(reader, type_id)?);
                }
                Self::TagList(arr)
            },
            10 => {
                let mut arr = Vec::new();
                loop {
                    let tag = NBT::from_reader(reader)?;
                    let should_break = tag.data == Self::TagEnd;
                    arr.push(tag);
                    if should_break {
                        break;
                    }
                }
                Self::TagCompound(arr)
            },
            11 => {
                let len = reader.read_int();
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(reader.read_int())
                }
                Self::TagIntArray(arr)
            },
            12 => {
                let len = reader.read_int();
                let mut arr = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    arr.push(reader.read_long());
                }
                Self::TagLongArray(arr)
            },
            id => return Err(Errors::InvalidField(format!("Invalid NBT id: {}", id)))
        })
    }
    fn to_inner_bytes(&self) -> Vec<u8> {
        match self {
            Tag::TagEnd => vec![0],
            Tag::TagByte(b) => vec![1, *b as u8],
            Tag::TagShort(short) => prefix!(2, encode_short(*short)),
            Tag::TagInt(int) => prefix!(3, encode_int(*int)),
            Tag::TagLong(long) => prefix!(4, encode_long(*long)),
            Tag::TagFloat(float) => prefix!(5, encode_float(*float)),
            Tag::TagDouble(double) => prefix!(6, encode_double(*double)),
            Tag::TagByteArray(arr) => prefix!(7, arr.iter().map(|b| *b as u8)),
            Tag::TagString(str) => {
                let len = encode_ushort(str.len() as UShort);
                let data = cesu8::to_java_cesu8(&*str).to_vec();
                vec![8].into_iter().chain(len).chain(data).collect::<Vec<u8>>()
            }
            Tag::TagList(arr) => {
                let len = encode_int(arr.len() as Int);
                let data = arr.iter().flat_map(|t| t.to_bytes());
                len.into_iter().chain(data).collect::<Vec<u8>>()
            },
            Tag::TagCompound(arr) => 
                encode_int(arr.len() as Int)
                    .into_iter()
                    .chain(arr.iter().flat_map(|nbt| nbt.to_bytes()))
                    .collect::<Vec<u8>>(),
            Tag::TagIntArray(arr) => 
                encode_int(arr.len() as Int)
                    .into_iter()
                    .chain(arr.iter().flat_map(|int| encode_int(*int)))
                    .collect(),
            Tag::TagLongArray(arr) => 
                encode_int(arr.len() as Int)
                    .into_iter()
                    .chain(arr.iter().flat_map(|long| encode_long(*long)))
                    .collect(),
        }
    }
    fn get_id(&self) -> u8 {
        match self {
            Tag::TagEnd => 0,
            Tag::TagByte(_) => 1,
            Tag::TagShort(_) => 2,
            Tag::TagInt(_) => 3,
            Tag::TagLong(_) => 4,
            Tag::TagFloat(_) => 5,
            Tag::TagDouble(_) => 6,
            Tag::TagByteArray(_) => 7,
            Tag::TagString(_) => 8,
            Tag::TagList(_) => 9,
            Tag::TagCompound(_) => 10,
            Tag::TagIntArray(_) => 11,
            Tag::TagLongArray(_) => 12,
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        prefix!(self.get_id(), self.to_inner_bytes())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NBT {
    pub data: Tag,
    pub name: String,
}
impl Field for NBT {
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.data.get_id()].into_iter()
            .chain(encode_ushort(self.name.len() as UShort))
            .chain(cesu8::to_java_cesu8(&*self.name).to_vec())
            .chain(self.data.to_inner_bytes())
            .collect::<Vec<u8>>()
    }

    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let type_id = reader.read_ubyte();
        if type_id == 0 {
            return Ok(Self { data: Tag::TagEnd, name: "".to_string() })
        }
        let name_length = reader.read_ushort();
        let buf = reader.read_byte_array(name_length as usize);
        let name = cesu8::from_java_cesu8(&*buf)?.to_string();

        let data = Tag::from_reader_with_type(reader, type_id)?;
        Ok(Self { data, name })
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NetworkNBT(Vec<NBT>);
impl Field for NetworkNBT {
    fn to_bytes(&self) -> Vec<u8> {
        Tag::TagCompound(self.0.clone()).to_bytes()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let t = reader.read_ubyte();
        if t != 10 {
            return Err(Errors::InvalidField(format!("NetworkNBT is expected to start with Compound Tag (id 10). Found tag with id {} instead", t)))
        }
        let Tag::TagCompound(tags) = Tag::from_reader_with_type(reader, 10)? else {
            unreachable!()
        };
        println!("{:?}", tags);
        Ok(Self(tags))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextComponent(Tag);
impl Field for TextComponent {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let tag_type = reader.read_byte();
        match tag_type {
            8 => Ok(Self(Tag::from_reader_with_type(reader, 8)?)),
            10 => Ok(Self(Tag::from_reader_with_type(reader, 10)?)),
            _ => panic!("invalid tag type for TextComponent: {tag_type}")
        }
    }
}