use crate::fields::{encode_float, encode_string, encode_ubyte, encode_uuid, encode_var_int};
use crate::{Errors, Packet, PacketReader, Result};

#[derive(Debug, Clone)]
pub struct BossBarAdd {
    title: String,
    health: f32,
    color: i32,
    division: i32,
    flags: u8,
}
impl BossBarAdd {
    pub fn new(title: String, health: f32, color: i32, division: i32, flags: u8) -> Self {
        Self {title, health, color, division, flags}
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_string(self.title.clone()),
            encode_float(self.health),
            encode_var_int(self.color),
            encode_var_int(self.division),
            encode_ubyte(self.flags),
        ].iter().flatten().cloned().collect()
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            title: reader.read_string()?,
            health: reader.read_float(),
            color: reader.read_var_int()?,
            division: reader.read_var_int()?,
            flags: reader.read_ubyte(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BossBarUpdateStyle {
    color: i32,
    dividers: i32,
}

impl BossBarUpdateStyle {
    pub fn new(color: i32, dividers: i32) -> Self {
        Self {color, dividers}
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.color),
            encode_var_int(self.dividers),
        ].iter().flatten().cloned().collect()
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            color: reader.read_var_int()?,
            dividers: reader.read_var_int()?,
        })
    }
}


#[derive(Debug, Clone)]
pub enum BossBarActions {
    Add(BossBarAdd),
    Remove,
    UpdateHealth(f32),
    UpdateTitle(String),
    UpdateStyle(BossBarUpdateStyle),
    UpdateFlags(u8),
}



#[derive(Debug)]
pub struct BossBar {
    uuid: u128,
    action: i32,
    action_data: BossBarActions,
}

impl Packet for BossBar {
    const ID: i32 = 0x0A;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_uuid(self.uuid);
        res.append(&mut encode_var_int(self.action));
        match self.action_data.clone() {
            BossBarActions::Add(dat) => res.append(&mut dat.to_bytes()),
            BossBarActions::Remove => (),
            BossBarActions::UpdateHealth(dat) => res.append(&mut encode_float(dat)),
            BossBarActions::UpdateTitle(dat) => res.append(&mut encode_string(dat)),
            BossBarActions::UpdateStyle(dat) => res.append(&mut dat.to_bytes()),
            BossBarActions::UpdateFlags(dat) => res.push(dat),
        }
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let uuid = reader.read_uuid();
        let action = reader.read_var_int()?;
        let action_data = match action {
            0 => BossBarActions::Add(BossBarAdd::from_reader(reader)?),
            1 => BossBarActions::Remove,
            2 => BossBarActions::UpdateHealth(reader.read_float()),
            3 => BossBarActions::UpdateTitle(reader.read_string()?),
            4 => BossBarActions::UpdateStyle(BossBarUpdateStyle::from_reader(reader)?),
            5 => BossBarActions::UpdateFlags(reader.read_ubyte()),
            _ => return Err(Errors::InvalidPacket("Boss Bar Action Invalid".into()))
        };
        Ok(BossBar {uuid, action, action_data})
    }
}
impl BossBar {
    pub fn new(uuid: u128, action: i32, action_data: BossBarActions) -> Self {
        Self { uuid, action, action_data }
    }
}