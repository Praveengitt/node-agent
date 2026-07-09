mod collector;
mod inventory;
mod models;
mod metrics;

use std::{
    thread,
    time::Duration,
};

use sysinfo::System;

fn main() {
    
    let mut system = System::new_all();

    loop{

        system.refresh_all();

        let report = collector::collect_report(&system);

        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap()
        );

        println!("---------------------------------------");

        thread::sleep(Duration::from_secs(5));

    }


}