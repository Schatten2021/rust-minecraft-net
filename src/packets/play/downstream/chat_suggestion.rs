use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x18]
pub struct ChatSuggestion {
    action: i32,
    count: i32,
    #[len = "count"]
    entries: Vec<String>,
}

impl ChatSuggestion {
    pub fn new(action: i32, entries: Vec<String>) -> Self {
        Self {
            action,
            count: entries.len() as i32,
            entries,
        }
    }
}