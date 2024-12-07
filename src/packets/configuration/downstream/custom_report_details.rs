use minecraft_net_proc::{Field, Packet};


#[derive(Debug, Field, Clone)]
pub struct ReportDetail {
    pub title: String,
    pub description: String,
}
impl ReportDetail {
    pub fn new(title: String, description: String) -> Self {
        Self {title, description}
    }
}
#[derive(Debug, Packet, Clone)]
#[id = 0x0F]
pub struct CustomReportDetails {
    #[Var]
    pub count: i32,
    #[len = "count"]
    pub details: Vec<ReportDetail>,
}
impl CustomReportDetails {
    pub fn new(details: Vec<ReportDetail>) -> Self {
        Self {
            count: details.len() as i32,
            details,
        }
    }
}