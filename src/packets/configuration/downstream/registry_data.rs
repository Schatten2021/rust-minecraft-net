use minecraft_net_proc::{Field, Packet};

#[derive(Debug, Field, Clone)]
pub struct Entry {
    pub entry_id: String,
    pub has_data: bool,
    #[when = "has_data"]
    pub data: Option<crab_nbt::Nbt>,
}
impl Entry {
    pub fn new(id: String, data: Option<crab_nbt::Nbt>) -> Self {
        Self {
            entry_id: id,
            has_data: data.is_none(),
            data,
        }
    }
}

#[derive(Debug, Packet)]
#[id = 0x07]
pub struct RegistryData {
    pub registry_id: String,
    #[Var]
    pub entry_count: i32,
    #[len = "entry_count"]
    pub entries: Vec<Entry>
}
impl RegistryData {
    pub fn new(id: String, entries: Vec<Entry>) -> Self {
        Self {
            registry_id: id,
            entry_count: entries.len() as i32,
            entries,
        }
    }
}