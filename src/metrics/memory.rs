use sysinfo::System;

pub fn get_memory(system: &System) -> (u64, u64) {

    (
        system.used_memory() / 1024 / 1024,
        system.total_memory() / 1024 / 1024,
    )
}