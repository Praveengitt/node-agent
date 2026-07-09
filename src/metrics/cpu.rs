use sysinfo::System;

pub fn get_cpu_usage(system: &System) -> f32 {
    let cpus = system.cpus();

    cpus.iter()
        .map(|cpu|cpu.cpu_usage())
        .sum::<f32>()
        / cpus.len() as f32
}