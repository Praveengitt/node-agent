use std::fs;

pub fn get_serial_number() -> String {
    let serial_path = "/sys/class/dmi/id/product_serial";
    fs::read_to_string(serial_path)
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string()
}