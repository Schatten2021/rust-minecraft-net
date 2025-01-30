use minecraft_net_proc::Field;
use crate::{Field, PacketReader};
use crate::fields::{encode_int, encode_long, encode_string, encode_var_int};
use crate::fields::types::{Identifier, Int, VarInt};

Field!(BitSet, {
    data: PrefixedArray<Long>
});
impl BitSet {
    pub fn is_set(&self, i: usize) -> bool {
        self.data[i / 64] & (1 << (i % 64)) != 0
    }
    pub fn set(&mut self, i: usize) {
        self.extend_to(i);
        self.set_unchecked(i);
    }
    pub fn set_unchecked(&mut self, i: usize) {
        self.data[i / 64] |= 1 << (i % 64);
    }
    pub fn reset(&mut self, i: usize) {
        self.extend_to(i);
        self.reset_unchecked(i);
    }
    pub fn reset_unchecked(&mut self, i: usize) {
        self.data[i / 64] &= !(1 << (i % 64));
    }
    pub fn extend_to(&mut self, i: usize) {
        let to_add = (i / 8) + if i % 8 != 0 {1} else {0};
        self.data.resize(to_add, 0);
    }
}
#[derive(Clone, Debug)]
pub struct IdOr<T: Field + Clone> {
    pub id: VarInt,
    pub value: Option<T>,
}
impl<T: Field + Clone> Field for IdOr<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.id);
        if self.id == 0 {
            res.append(&mut self.value.clone().expect("invalid IdOr field. Requires either ID to be non-zero or a value").to_bytes());
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        let id = reader.read_var_int()?;
        if id == 0 {
            Ok(Self {
                id,
                value: Some(T::from_reader(reader)?),
            })
        } else {
            Ok(Self {
                id, value: None,
            })
        }
    }
}
impl<T: Clone + Field> IdOr<T> {
    pub fn with_id(id: VarInt) -> Self {
        Self {id, value: None}
    }
    pub fn with_value(value: T) -> Self {
        Self {id: 0, value: Some(value) }
    }
}
#[derive(Clone, Debug)]
pub enum IDSet {
    TagName(Identifier),
    Ids(Vec<VarInt>),
}
impl Field for IDSet {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            IDSet::TagName(name) => vec![encode_var_int(0), encode_string(name.clone())].iter().flatten().cloned().collect::<Vec<u8>>(),
            IDSet::Ids(ids) => vec![
                encode_var_int(ids.len() as i32 + 1),
                ids.iter().flat_map(|id| encode_var_int(*id)).collect::<Vec<u8>>(),
            ].iter().flatten().cloned().collect::<Vec<u8>>(),
        }
    }
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        let t = reader.read_var_int()?;
        if t == 0 {
            Ok(Self::TagName(reader.read_identifier()?))
        } else {
            let mut res: Vec<VarInt> = Vec::with_capacity((t - 1) as usize);
            for _ in 0..(t-1) {
                res.push(reader.read_var_int()?);
            }
            Ok(Self::Ids(res))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TeleportFlags(Int);
impl TeleportFlags {
    pub fn new(relative_x: bool,
               relative_y: bool,
               relative_z: bool,
               relative_yaw: bool,
               relative_pitch: bool,
               relative_velocity_x: bool,
               relative_velocity_y: bool,
               relative_velocity_z: bool,
               rotation_first: bool) -> Self {
        let mut new = Self::default();
        new.set_relative_x(relative_x);
        new.set_relative_y(relative_y);
        new.set_relative_z(relative_z);
        new.set_relative_yaw(relative_yaw);
        new.set_relative_pitch(relative_pitch);
        new.set_relative_velocity_x(relative_velocity_x);
        new.set_relative_velocity_y(relative_velocity_y);
        new.set_relative_velocity_z(relative_velocity_z);
        new.set_rotation_first(rotation_first);
        new
    }
    fn apply_mask(&mut self, index: usize, value: bool) {
        if value {
            self.0 |= 1 << index;
        } else {
            self.0 &= !(1 << index);
        }
    }
    pub fn set_relative_x(&mut self, new: bool) -> () {
        self.apply_mask(0, new)
    }
    pub fn set_relative_y(&mut self, new: bool) -> () {
        self.apply_mask(1, new)
    }
    pub fn set_relative_z(&mut self, new: bool) -> () {
        self.apply_mask(2, new)
    }
    pub fn set_relative_yaw(&mut self, new: bool) -> () {
        self.apply_mask(3, new)
    }
    pub fn set_relative_pitch(&mut self, new: bool) -> () {
        self.apply_mask(4, new)
    }
    pub fn set_relative_velocity_x(&mut self, new: bool) -> () {
        self.apply_mask(5, new)
    }
    pub fn set_relative_velocity_y(&mut self, new: bool) -> () {
        self.apply_mask(6, new)
    }
    pub fn set_relative_velocity_z(&mut self, new: bool) -> () {
        self.apply_mask(7, new)
    }
    pub fn set_rotation_first(&mut self, new: bool) -> () {
        self.apply_mask(8, new)
    }
}
impl Field for TeleportFlags {
    fn to_bytes(&self) -> Vec<u8> {
        encode_int(self.0)
    }

    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Self> {
        Ok(Self(reader.read_int()))
    }
}
impl Default for TeleportFlags {
    fn default() -> Self {
        Self(0)
    }
}

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
    fn from_reader(reader: &mut PacketReader) -> crate::errors::Result<Position> {
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