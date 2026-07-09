mod collector;
mod inventory;
mod models;
mod metrics;
mod http;

use std::{
    thread,
    time::Duration,
};

use sysinfo::System;

fn main() {
    
    let mut system = System::new_all();

    let report = collector::collect_report(&system);

    match http::client::register(&report) {
        Ok(_) => println!("Node report registered successfully."),
        Err(e) => eprintln!("Error registering node report: {}", e),
    }


   /* loop{

        system.refresh_all();

        let report = collector::collect_report(&system);

        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap()
        );

        println!("---------------------------------------");

        thread::sleep(Duration::from_secs(5));

    } */


}