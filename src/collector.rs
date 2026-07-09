use crate::inventory::{
    hostname,
    machine_id,
    serial,
};

use crate::metrics::{
    cpu,
    memory,
    uptime,
};

use sysinfo::System;

use crate::models::NodeIdentity;
use crate::models::NodeMetrics;

pub fn collect_identity() -> NodeIdentity {

    NodeIdentity {

        hostname: hostname::get_hostname(),

        machine_id: machine_id::get_machine_id(),

        serial_number: serial::get_serial_number(),
    }
}

pub fn collect_metrics(system: &System) -> NodeMetrics {

    let (used, total) = memory::get_memory(system);

    NodeMetrics {

        cpu_usage: cpu::get_cpu_usage(system),

        memory_used_mb: used,

        memory_total_mb: total,

        uptime_secs: uptime::get_uptime(),
    }
}