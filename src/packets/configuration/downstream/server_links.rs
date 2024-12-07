use crate::fields::{encode_bool, encode_string, encode_var_int};
use crate::{Field, PacketReader, Result};
use minecraft_net_proc::Packet;

#[derive(Debug, Clone)]
pub enum LinkLabel {
    Builtin(i32),
    Other(String),
}
#[derive(Debug, Clone)]
pub struct Link {
    pub is_builtin: bool,
    pub label: LinkLabel,
    pub url: String,
}
impl Link {
    pub fn new(label: LinkLabel, url: String) -> Self {
        Self {
            is_builtin: match label {
                LinkLabel::Builtin(_) => true,
                LinkLabel::Other(_) => false
            },
            label,
            url,
        }
    }
}
impl Field for Link {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_bool(self.is_builtin);
        res.append(&mut match &self.label {
            LinkLabel::Builtin(v) => encode_var_int(v.clone()),
            LinkLabel::Other(s) => encode_string(s.clone()),
        });
        res.append(&mut encode_string(self.url.clone()));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
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
}
#[derive(Debug, Packet)]
#[id = 0x10]
pub struct ServerLinks {
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub links: Vec<Link>,
}
impl ServerLinks {
    pub fn new(links: Vec<Link>) -> Self {
        Self { count: links.len() as i32, links }
    }
}
