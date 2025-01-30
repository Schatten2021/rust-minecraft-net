use crate::fields::{encode_bool, encode_string, encode_var_int};
use crate::{Field, PacketReader, Result};
use minecraft_net_proc::Packet;
use crate::fields::types::TextComponent;

#[derive(Debug, Clone)]
pub enum LinkLabel {
    Builtin(i32),
    Custom(TextComponent),
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
                LinkLabel::Custom(_) => false
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
            LinkLabel::Custom(s) => s.to_bytes(),
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
                label: LinkLabel::Custom(reader.read()?),
                url: reader.read_string()?,
            })
        }
    }
}
Packet!(ServerLinks, 0x10, {
    links: PrefixedArray<Link>,
});