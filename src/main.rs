mod http;
mod collector;
mod inventory;
mod metrics;
mod models;



use sysinfo::System;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    println!("=================================");
    println!("Node Agent Starting...");
    println!("=================================");

    let mut system = System::new_all();

    loop {
        // Refresh Linux metrics
        system.refresh_all();

        // Build complete report
        let report = collector::collect_report(&system);

        // Send report to Inventory Server
        match http::client::register_device(&report).await {
            Ok(_) => println!("Report sent successfully."),
            Err(e) => eprintln!("Failed to send report: {}", e),
        }

        sleep(Duration::from_secs(5)).await;
    }
}