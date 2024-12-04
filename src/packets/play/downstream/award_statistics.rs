use crate::{Packet, PacketReader, Result};
use crate::fields::encode_var_int;

#[derive(Debug)]
pub struct Statistic {
    pub category_id: i32,
    pub statistics_id: i32,
    pub value: i32,
}
impl Statistic {
    pub fn new(category_id: i32, statistics_id: i32, value: i32) -> Self {
        Self {category_id, statistics_id, value}
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![
            encode_var_int(self.category_id),
            encode_var_int(self.statistics_id),
            encode_var_int(self.value),
        ].iter().flatten().cloned().collect()
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self {
            category_id: reader.read_var_int()?,
            statistics_id: reader.read_var_int()?,
            value: reader.read_var_int()?,
        })
    }
}

#[derive(Debug)]
pub struct AwardStatistics {
    pub count: i32,
    pub statistics: Vec<Statistic>
}
impl Packet for AwardStatistics {
    const ID: i32 = 0x04;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.count);
        res.append(&mut self.statistics.iter().flat_map(|s| s.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        let mut statistics = Vec::with_capacity(count as usize);
        for _ in 0..count {
            statistics.push(Statistic::from_reader(reader)?);
        }
        Ok(Self {
            count,
            statistics,
        })
    }
}
impl AwardStatistics {
    pub fn new(statistics: Vec<Statistic>) -> Self {
        Self {
            count: statistics.len() as i32,
            statistics,
        }
    }
}