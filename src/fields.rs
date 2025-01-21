use crate::errors::Errors;
use bytes::Buf;
use std::io::Read;
use crate::Field;
use crate::fields::types::PrefixedArray;

pub mod types {
    pub type Byte = i8;
    pub type UByte = u8;
    pub type Short = i16;
    pub type UShort = u16;
    pub type Int = i32;
    pub type UInt = u32;
    pub type Long = i32;
    pub type UUID = u128;
    pub type Float = f32;
    pub type Double = f64;
    pub type Identifier = String;
    pub type Angle = u8;
    pub type VarInt = Int;
    pub type VarLong = Long;
    pub type PrefixedArray<T> = Vec<T>;
    pub type PrefixedOptional<T> = Option<T>;
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub fn encode_bool(boolean: bool) -> Vec<u8> {
    vec![if boolean {1} else {0}]
}
pub fn encode_ubyte(ubyte: u8) -> Vec<u8> {vec![ubyte]}
pub fn encode_byte(byte: i8) -> Vec<u8> {vec![byte as u8]}
pub fn encode_short(short: i16) -> Vec<u8> {vec![(short >> 8) as u8, short as u8]}
pub fn encode_ushort(ushort: u16) -> Vec<u8> {
    vec![(ushort >> 8) as u8, ushort as u8]
}
pub fn encode_int(int: i32) -> Vec<u8> {
    vec![
        (int >> 24) as u8,
        (int >> 16) as u8,
        (int >> 8) as u8,
        int as u8,
    ]
}
pub fn encode_uint(uint: u32) -> Vec<u8> {
    vec![
        (uint >> 24) as u8,
        (uint >> 16) as u8,
        (uint >> 8) as u8,
        uint as u8,
    ]
}
pub fn encode_long(long: i64) -> Vec<u8> {
    vec![
        (long >> 56) as u8,
        (long >> 48) as u8,
        (long >> 40) as u8,
        (long >> 32) as u8,
        (long >> 24) as u8,
        (long >> 16) as u8,
        (long >> 8) as u8,
        long as u8,
    ]
}
pub fn encode_uuid(uuid: u128) -> Vec<u8> {
    let mut res = Vec::new();
    for i in 0..16 {
        res.push((uuid >> ((15 - i) * 8) ) as u8)
    }
    res
}

pub fn encode_float(float: f32) -> Vec<u8> {
    encode_uint(float.to_bits())
}
pub fn encode_double(double: f64) -> Vec<u8> {
    encode_long(double.to_bits() as i64)
}

pub fn read_var_int_from_stream(stream: &mut impl Read) -> Result<i32, Errors> {
    let mut value: i32 = 0;
    let mut buff: Vec<u8> = Vec::new();
    buff.resize(1, 0);
    let mut position = 0;
    loop {
        stream.read(&mut *buff).map_err(|e| Errors::IOError(e))?;
        let byte = buff[0];
        value |= ((byte & SEGMENT_BITS) as i32) << position;
        if (byte & CONTINUE_BIT) == 0 {
            return Ok(value);
        }
        position += 7;
        if position >= 32 {
            return Err(Errors::InvalidField("Invalid VarInt".into()));
        }
    }
}
pub fn encode_var_int(int: i32) -> Vec<u8> {
    if int == 0 {
        return vec![0];
    }
    let mut result = Vec::new();
    let mut remaining = int;
    loop {
        let mut segment = (remaining as u8) & SEGMENT_BITS;
        let cont = (segment as i32) != remaining;
        if cont {
            segment |= CONTINUE_BIT;
        }
        result.push(segment);
        if !cont {
            return result;
        }
        remaining >>= 7;
    }
}
pub fn encode_var_long(long: i64) -> Vec<u8> {
    if long == 0 {
        return vec![0];
    }
    let mut result = Vec::new();
    let mut remaining = long;
    loop {
        let mut segment = (remaining as u8) & SEGMENT_BITS;
        let cont = (segment as i64) != remaining;
        if cont {
            segment |= CONTINUE_BIT;
        }
        result.push(segment);
        if !cont {
            return result;
        }
        remaining >>= 7;
    }
}
pub fn var_int_size(var_int: i32) -> usize {
    encode_var_int(var_int).len()
}

pub fn encode_string(string: String) -> Vec<u8> {
    let mut bytes = string.into_bytes();
    let mut res = encode_var_int(bytes.len() as i32);
    res.append(&mut bytes);
    res
}
pub fn encode_identifier(ident: String) -> Vec<u8> {
    encode_string(ident)
}
pub fn encode_angle(angle: u8) -> Vec<u8> {encode_ubyte(angle)}
pub fn encode_prefixed_array<T: Field>(array: &Vec<T>) -> Vec<u8> {
    let mut res = encode_var_int(array.len() as i32);
    res.append(&mut array.iter().flat_map(|e| e.to_bytes()).collect::<Vec<u8>>());
    res
}
pub fn encode_prefixed_optional<T: Field + Clone>(optional: &Option<T>) -> Vec<u8> {
    let mut res = encode_bool(optional.is_some());
    match optional {
        Some(v) => res.append(&mut T::to_bytes(v)),
        None => (),
    }
    res
}



pub struct PacketReader {
    data: Vec<u8>,
    position: usize,
}

impl PacketReader {
    pub fn get_rest(&self) -> Vec<u8> {
        self.data[self.position..].into()
    }
}

impl PacketReader {
    pub fn read_bool(&mut self) -> Result<bool, Errors> {
        match self.read() {
            0x00 => Ok(false),
            0x01 => Ok(true),
            val => Err(Errors::InvalidField(format!("boolean isn't 0x00 or 0x01 but {}", val))),
        }
    }
    pub fn read_ubyte(&mut self) -> u8 { self.read() }
    pub fn read_byte(&mut self) -> i8 { self.read() as i8 }
    pub fn read_ushort(&mut self) -> u16 {
        self.concat(2) as u16
    }
    pub fn read_short(&mut self) -> i16 { self.concat(2) as i16 }
    pub fn read_uint(&mut self) -> u32 {
        self.concat(4) as u32
    }
    pub fn read_int(&mut self) -> i32 {
        let arr = self.read_n(4);
        (arr[0] as i32) << 24 |
            (arr[1] as i32) << 16 |
            (arr[2] as i32) << 8 |
            (arr[3] as i32)
    }
    pub fn read_long(&mut self) -> i64 {
        let arr = self.read_n(8);
        assert_eq!(arr.len(), 8);
        let mut res = 0;
        for i in 0..8 {
            res |= (arr[i] as i64) << ((7 - i) * 8) 
        }
        res
    }
    pub fn read_uuid(&mut self) -> u128 {
        self.concat(16)
    }
    
    pub fn read_float(&mut self) -> f32 {f32::from_bits(self.read_uint())}
    pub fn read_double(&mut self) -> f64 {
        f64::from_bits(self.read_long() as u64)
    }
    
    pub fn read_var_int(&mut self) -> Result<i32, Errors> {
        let mut value: i32 = 0;
        let mut position = 0;
        loop {
            let byte = self.read();
            value |= ((byte & SEGMENT_BITS) as i32) << position;
            if (byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }
            position += 7;
            if position >= 32 {
                return Err(Errors::InvalidField("Invalid VarInt".into()));
            }
        }
    }
    pub fn read_var_long(&mut self) -> Result<i64, Errors> {
        let mut value: i64 = 0;
        let mut position = 0;
        loop {
            let byte = self.read();
            value |= ((byte & SEGMENT_BITS) as i64) << position;
            if (byte & CONTINUE_BIT) == 0 {
                return Ok(value);
            }
            position += 7;
            if position >= 64 {
                return Err(Errors::InvalidField("Invalid VarInt".into()));
            }
        }
    }
    pub fn read_string(&mut self) -> Result<String, Errors> {
        let length = self.read_var_int()?;
        let data = self.read_n(length as usize);
        let string = String::from_utf8(data).map_err(|_| Errors::InvalidField("String isn't valid UTF-8".into()))?;
        Ok(string)
    }
    pub fn read_identifier(&mut self) -> Result<String, Errors> {
        self.read_string()
    }
    pub fn read_byte_array(&mut self, length: usize) -> Vec<u8> {
        self.read_n(length)
    }
    pub fn read_rest(&mut self) -> Vec<u8> {
        let res = self.data[self.position..].iter().cloned().collect();
        self.position = self.data.len();
        res
    }
    pub fn read_angle(&mut self) -> u8 {self.read_ubyte()}
    pub fn read_prefixed_array<T: Field>(&mut self) -> Result<PrefixedArray<T>, Errors> {
        let len = self.read_var_int()?;
        let mut vector = Vec::new();
        for _ in 0..len {
            vector.push(T::from_reader(self)?);
        }
        Ok(vector)
    }
    pub fn read_prefixed_optional<T: Field>(&mut self) -> Result<Option<T>, Errors> {
        let present = self.read_bool()?;
        if present {
            Ok(Some(T::from_reader(self)?))
        } else {
            Ok(None)
        }
    }
}
impl Buf for PacketReader {
    fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    fn chunk(&self) -> &[u8] {
        &self.data[self.position..]
    }

    fn advance(&mut self, cnt: usize) {
        self.position += cnt
    }
}
impl PacketReader {
    fn read(&mut self) -> u8 {
        let value = self.data[self.position];
        self.position += 1;
        value
    }
    fn read_n(&mut self, n: usize) -> Vec<u8> {
        let value: Vec<u8> = self.data[self.position..(self.position + n)].iter().cloned().collect();
        self.position += n;
        value
    }
    fn concat(&mut self, size: usize) -> u128 {
        let mut result: u128 = 0;
        for i in 0..size {
            result |= (self.read() as u128) << ((size - i - 1) * 8)
        }
        result
    }
}
impl PacketReader {
    pub fn total_len(&self) -> usize {
        self.data.len()
    }
    pub fn len(&self) -> usize {
        self.data.len() - self.position
    }
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
        }
    }
}