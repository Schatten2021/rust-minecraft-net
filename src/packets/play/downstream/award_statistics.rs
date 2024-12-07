use minecraft_net_proc::{Field, Packet};

#[derive(Debug, Field, Clone)]
pub struct Statistic {
    #[Var]
    pub category_id: i32,
    #[Var]
    pub statistics_id: i32,
    #[Var]
    pub value: i32,
}

#[derive(Debug, Packet)]
#[id = 0x04]
pub struct AwardStatistics {
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub statistics: Vec<Statistic>
}
impl AwardStatistics {
    pub fn new(statistics: Vec<Statistic>) -> Self {
        Self {
            count: statistics.len() as i32,
            statistics,
        }
    }
}