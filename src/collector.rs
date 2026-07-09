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

    NodeMetrics {

        cpu: cpu::get_cpu_usage(system)

        memory: memory::get_memory(system)

        uptime: uptime::get_uptime(system)
    }
}