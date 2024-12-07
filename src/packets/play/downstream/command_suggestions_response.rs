use minecraft_net_proc::{Field, Packet};


#[derive(Debug, Field, Clone)]
pub struct Matches {
    pub r#match: String,
    pub has_tooltip: bool,
    #[when = "has_tooltip"]
    pub tooltip: Option<String>
}
impl Matches {
    pub fn new(r#match: String, tooltip: Option<String>) -> Self {
        Self {
            r#match,
            has_tooltip: tooltip.is_some(),
            tooltip,
        }
    }
}

#[derive(Debug, Packet)]
#[id = 0x10]
pub struct CommandSuggestionResponse {
    #[Var]
    pub id: i32,
    #[Var]
    pub start: i32,
    #[Var]
    pub length: i32,
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub matches: Vec<Matches>
}
impl CommandSuggestionResponse {
    pub fn new(id: i32, start: i32, length: i32, matches: Vec<Matches>) -> Self {
        Self {id, start, length, count: matches.len() as i32, matches}
    }
}