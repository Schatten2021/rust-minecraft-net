use minecraft_net_proc::Packet;

#[derive(Debug, Packet)]
#[id = 0x0B]
pub struct ChangeDifficulty {
    difficulty: u8,
    difficulty_locked: bool,
}
impl ChangeDifficulty {
    pub fn new(difficulty: u8, locked: bool) -> Self {
        Self {difficulty, difficulty_locked: locked}
    }
}