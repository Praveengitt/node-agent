use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct NodeIdentity {
    pub hostname: String,
    pub machine_id: String,
    pub serial_number: String,
}