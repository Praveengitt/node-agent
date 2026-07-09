use reqwest::blocking::Client;
use crate::models::NodeReport;

pub fn register(report: &NodeReport) -> Result<(), reqwest::Error> {
    let client = Client::new();
    
    let response = client.post("http://localhost:8080/register")
        .json(report)
        .send()?;
    
    if response.status().is_success() {
        println!("Successfully registered the node.");
        Ok(())
    } else {
        eprintln!("Failed to register the node. Status: {}", response.status());
        Err(reqwest::Error::new(reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(), "Failed to register"))
    }
}
