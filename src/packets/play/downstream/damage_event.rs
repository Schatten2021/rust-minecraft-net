use crate::{Packet, PacketReader, Result};
use crate::fields::{encode_bool, encode_byte, encode_double, encode_var_int};

#[derive(Debug)]
pub struct DamageEvent {
    pub entity_id: i32,
    pub source_type_id: i32,
    pub source_cause_id: i32,
    pub source_direct_id: i32,
    pub has_source_position: bool,
    pub source_position_x: Option<f64>,
    pub source_position_y: Option<f64>,
    pub source_position_z: Option<f64>,
}
impl Packet for DamageEvent {
    const ID: i32 = 0x1A;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![
            encode_var_int(self.entity_id),
            encode_var_int(self.source_type_id),
            encode_var_int(self.source_cause_id),
            encode_var_int(self.source_direct_id),
            encode_bool(self.has_source_position),
        ];
        if self.has_source_position {
            res.append(&mut vec![
                encode_double(self.source_position_x.unwrap()),
                encode_double(self.source_position_y.unwrap()),
                encode_double(self.source_position_z.unwrap()),
            ])
        }
        res.iter().flatten().cloned().collect()
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let entity_id = reader.read_var_int()?;
        let source_type_id = reader.read_var_int()?;
        let source_cause_id = reader.read_var_int()?;
        let source_direct_id = reader.read_var_int()?;
        let has_source_position = reader.read_bool()?;
        if has_source_position {
            Ok(Self {
                entity_id, source_type_id, source_cause_id, source_direct_id, has_source_position,
                source_position_x: Some(reader.read_double()),
                source_position_y: Some(reader.read_double()),
                source_position_z: Some(reader.read_double()),
            })
        } else {
            Ok(Self {
                entity_id, source_type_id, source_cause_id, source_direct_id, has_source_position,
                source_position_x: None,
                source_position_y: None,
                source_position_z: None,
            })
        }
    }
}
impl DamageEvent {
    pub fn new(entity_id: i32,
               source_type_id: i32,
               source_cause_id: i32,
               source_direct_id: i32,
               has_source_position: bool,
               source_position_x: Option<f64>,
               source_position_y: Option<f64>,
               source_position_z: Option<f64>,) -> Self {
        Self {entity_id, source_type_id, source_cause_id, source_direct_id, has_source_position, source_position_x, source_position_y, source_position_z, }
    }
}