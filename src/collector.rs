use crate::inventory::{
    hostname,
    machine_id,
    serial,
};

use crate::models::NodeIdentity;

pub fn collect_identity() -> NodeIdentity {

    NodeIdentity {

        hostname: hostname::get_hostname(),

        machine_id: machine_id::get_machine_id(),

        serial_number: serial::get_serial_number(),
    }
}