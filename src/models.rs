use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct NodeIdentity {
    pub hostname: String,
    pub machine_id: String,
    pub serial_number: String,
}

#[derive(Serialize, Debug)]
pub struct NodeMetrics {
    pub cpu_usage: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub uptime_secs: u64,

}