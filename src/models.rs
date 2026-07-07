use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct NodeMetrics {
    pub hostname: String
    pub machine_id: String,
}