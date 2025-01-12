use dotenv::dotenv;

use rust_db_manager_core::commons::configuration::configuration::Configuration;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Configuration::initialize().await;
    if let Err(err) = config {
        panic!("{}", err.to_string());
    }
    println!("rust-db-manager!");
}