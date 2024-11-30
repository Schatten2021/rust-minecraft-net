
use crate::fields::{encode_string, encode_var_int};
use crate::{Packet, PacketReader, Result};
#[derive(Debug)]
pub struct ReportDetail {
    pub title: String,
    pub description: String,
}
impl ReportDetail {
    pub fn new(title: String, description: String) -> Self {
        Self {title, description}
    }
    pub fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        Ok(Self::new(reader.read_string()?, reader.read_string()?))
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_string(self.title.clone());
        res.append(&mut encode_string(self.description.clone()));
        res
    }
}
#[derive(Debug)]
pub struct CustomReportDetails {
    pub count: i32,
    pub details: Vec<ReportDetail>,
}
impl Packet for CustomReportDetails {
    const ID: i32 = 0x0F;
    fn to_bytes(&self) -> Vec<u8> {
        let mut res = encode_var_int(self.count);
        res.append(&mut self.details.iter().flat_map(|d| d.to_bytes()).collect());
        res
    }
    fn from_reader(reader: &mut PacketReader) -> Result<Self> {
        let count = reader.read_var_int()?;
        let mut details = Vec::with_capacity(count as usize);
        for _ in 0..count {
            details.push(ReportDetail::from_reader(reader)?)
        }
        Ok(Self { count, details })
    }
}
impl CustomReportDetails {
    pub fn new(details: Vec<ReportDetail>) -> Self {
        Self {
            count: details.len() as i32,
            details,
        }
    }
}