use minecraft_net_proc::{Field, Packet};

Field!(ReportDetail, {
    title: String,
    description: String,
});
Packet!(CustomReportDetails, 0x0F, {
    details: PrefixedArray<ReportDetail>
});