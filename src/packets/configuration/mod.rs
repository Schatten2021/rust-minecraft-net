use minecraft_net_proc::Field_old;

pub mod upstream;
pub mod downstream;
#[derive(Debug, Field_old, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}
impl KnownPack {
    pub fn new(namespace: String, id: String, version: String) -> Self {
        Self {namespace, id, version}
    }
}
