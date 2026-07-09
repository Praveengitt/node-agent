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
    let identity = collector::collect_identity();
    println!("========== DEVICE IDENTITY ===========") ;

    println!(
        "{}",
        serde_json::to_string_pretty(&identity).unwrap()
    );

    println!();

    let mut system = System::new_all();

    loop{

        system.refresh_all();

        let metrics = collector::collect_metrics(&system);

        println!("=========== NODE METRICS ===========");

        println!(
            "{}",
            serde_json::to_string_pretty(&metrics).unwrap()
        );

        println!("---------------------------------------");

        thread::sleep(Duration::from_secs(5));

    }


}