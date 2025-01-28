use minecraft_net_proc::Packet;
use std::time::{SystemTime, UNIX_EPOCH};

Packet!(StatusRequest, 0x00, {});
Packet!(PingRequest, 0x01, {
    timestamp: Long
});

impl PingRequest {
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        Self {timestamp: now as i64}
    }
}