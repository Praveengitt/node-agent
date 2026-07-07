use crate::inventory::{
    hostname,
    machine_id,
};

use crate::models::NodeIdentity;

pub fn collect_identity() -> NodeIdentity {

    NodeIdentity {

        hostname: hostname::get_hostname(),

        machine_id: machine_id::get_machine_id(),
    }
}