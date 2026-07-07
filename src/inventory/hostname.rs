use sysinfo::System;

pub fn get_hostname() -> String {

    System::host_name()
        .unwrap_or_else(|| "unknown".to_string())
}