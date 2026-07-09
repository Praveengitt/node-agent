use std::fs;

pub fn get_serial_number() -> String {
    let serial_path = "/sys/class/dmi/id/product_serial";

    match fs::read_to_string(serial_path) {
        Ok(serial) => serial.trim().to_string(),
        Err(err) => {
            println!("Failed to read serial: {}", err);
            "unknown".to_string()
        }
    }
}