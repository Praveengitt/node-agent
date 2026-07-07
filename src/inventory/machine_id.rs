use std::fs;

pub fn get_machine_id() -> String {

    fs::read_to_string("/etc/machine-id")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}