use crate::{Result, PacketReader, Packet};
use crate::fields::{encode_bool, encode_string, encode_var_int};

//TODO: implement Packet
#[derive(Debug)]
pub enum LinkLabel {
    Builtin(i32),
    Other(String),
}
#[derive(Debug)]
pub struct Link {
    pub is_builtin: bool,
    pub label: LinkLabel,
    pub url: String,
}
impl Link {
    pub fn new(label: LinkLabel, url: String) -> Self {
        Self {
            is_builtin: match label {LinkLabel::Builtin(_) => true, LinkLabel::Other(_) => false},
            label,
            url,
        }
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let is_builtin = reader.read_bool()?;
        if is_builtin {
            Ok(Self {
                is_builtin,
                label: LinkLabel::Builtin(reader.read_var_int()?),
                url: reader.read_string()?,
            })
        } else {
            Ok(Self {
                is_builtin,
                label: LinkLabel::Other(reader.read_string()?),
                url: reader.read_string()?,
            })
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_bool(self.is_builtin);
        res.append(&mut match &self.label {
            LinkLabel::Builtin(v) => encode_var_int(v.clone()),
            LinkLabel::Other(s) => encode_string(s.clone()),
        });
        res.append(&mut encode_string(self.url.clone()));
        res
    }
}
#[derive(Debug)]
pub struct ServerLinks {
    pub count: i32,
    pub links: Vec<Link>,
}
impl Packet for ServerLinks {
    const ID: i32 = 0x10;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.count);
        res.append(&mut self.links.iter().flat_map(|l| l.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        let mut links = Vec::with_capacity(count as usize);
        for _ in 0..count {
            links.push(Link::from_reader(reader)?);
        }
        Ok(Self {count, links})
    }
}
