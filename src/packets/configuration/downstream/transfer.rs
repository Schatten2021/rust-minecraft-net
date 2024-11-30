use crate::fields::{encode_string, encode_var_int};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct Transfer {
    pub host: String,
    pub port: i32,
}
impl Packet for Transfer {
    const ID: i32 = 0x0B;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.host.clone());
        res.append(&mut encode_var_int(self.port));
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            host: reader.read_string()?,
            port: reader.read_var_int()?,
        })
    }
}
impl Transfer {
    pub  fn new(host: String, port: i32) -> Self {
        Self{host, port}
    }
}