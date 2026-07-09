use reqwest::Client;

use crate::models::NodeReport;

pub async fn register(report: &NodeReport) -> Result<(), reqwest::Error> {

    let client = Client::new();

    let response = client
        .post("http://localhost:8080/register")
        .json(report)
        .send()
        .await?;

    if response.status().is_success() {

        println!("Successfully registered the node.");

        Ok(())

    } else {

        eprintln!(
            "Failed to register the node. Status: {}",
            response.status()
        );

        Ok(())
    }
}