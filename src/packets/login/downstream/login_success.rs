use crate::fields::{encode_bool, encode_string, encode_uuid, encode_var_int};
use crate::{Errors, Packet, PacketReader};
//TODO: add Packet derive
#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>
}
impl Property {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Self {
            name, 
            value,
            is_signed: signature.is_none(),
            signature,
        }
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let name = reader.read_string()?;
        let value = reader.read_string()?;
        let is_signed = reader.read_bool()?;
        if is_signed {
            Ok(Self {
                name,
                value,
                is_signed,
                signature: Some(reader.read_string()?),
            })
        } else {
            Ok(Self { name, value, is_signed, signature: None })
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.name.clone());
        res.append(&mut encode_string(self.value.clone()));
        res.append(&mut encode_bool(self.is_signed));
        if self.is_signed {
            let signature = self.signature.clone().expect("is_signed is true but signature is none");
            res.append(&mut encode_string(signature));
        }
        res
    }
}
#[derive(Debug)]
pub struct LoginSuccess {
    pub uuid: u128,
    pub username: String,
    pub number_of_properties: i32,
    pub properties: Vec<Property>,
    pub strict_error_handling: bool,
}
impl Packet for LoginSuccess {
    const ID: i32 = 0x02;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_uuid(self.uuid);
        res.append(&mut encode_string(self.username.clone()));
        res.append(&mut encode_var_int(self.number_of_properties));
        res.append(&mut self.properties.iter().flat_map(|p| p.to_bytes()).collect());
        res.append(&mut encode_bool(self.strict_error_handling));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self, Errors> {
        let uuid = reader.read_uuid();
        let username = reader.read_string()?;
        let property_count = reader.read_var_int()?;
        let mut properties = Vec::with_capacity(property_count as usize);
        for _i in 0..property_count {
            properties.push(Property::from_reader(reader)?);
        }
        let strict_error_handling = reader.read_bool()?;
        Ok(Self {
            uuid,
            username,
            number_of_properties: property_count,
            properties,
            strict_error_handling,
        })
    }
}
impl LoginSuccess {
    pub fn new(uuid: u128, username: String, properties: Vec<Property>, strict_error_handling: bool) -> Self {
        Self {
            uuid, 
            username,
            number_of_properties: properties.len() as i32,
            properties,
            strict_error_handling,
        }
    }
}