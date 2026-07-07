use std::fs;

pub fn serial() -> Option<String> {
    fs::read_to_string("/sys/class/dmi/id/product_serial")
        .ok()
        .map(|s| s.trim().to_string())
}