use minecraft_net_proc::{Field, Packet};

#[derive(Debug, Field, Clone)]
pub struct Tag {
    pub tag_name: String,
    #[Var]
    pub count: i32,
    #[len = "count"]
    #[Var]
    pub entries: Vec<i32>
}
impl Tag {
    pub fn new(name: String, entries: Vec<i32>) -> Self {
        Self {
            tag_name: name,
            count: entries.len() as i32,
            entries,
        }
    }
}
#[derive(Debug, Field, Clone)]
pub struct Tags {
    pub registry: String,
    #[Var]
    pub length: i32,
    #[len = "length"]
    pub tags: Vec<Tag>
}
impl Tags {
    pub fn new(registry: String, tags: Vec<Tag>) -> Self {
        Self {registry, length: tags.len() as i32, tags}
    }
}
#[derive(Debug, Packet)]
#[id = 0x0D]
pub struct UpdateTags {
    #[Var]
    pub len: i32,
    #[len = "len"]
    pub arr: Vec<Tags>
}
impl UpdateTags {
    pub fn new(tags: Vec<Tags>) -> Self {
        Self {
            len: tags.len() as i32,
            arr: tags,
        }
    }
}