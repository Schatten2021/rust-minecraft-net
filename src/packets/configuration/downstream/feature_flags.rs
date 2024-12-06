use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x0C]
pub struct FeatureFlags {
    #[Var]
    pub total_features: i32,
    #[len = "total_features"]
    pub feature_flags: Vec<String>,
}
impl FeatureFlags {
    pub fn new(flags: Vec<String>) -> Self {
        Self {
            total_features: flags.len() as i32,
            feature_flags: flags
        }
    }
}