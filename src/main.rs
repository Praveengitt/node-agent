mod collector;
mod inventory;
mod models;

fn main() {
    let identity = collector::collect_identity();

    println!(
        "{}",
        serde_json::to_string_pretty(&identity).unwrap()
    );


}