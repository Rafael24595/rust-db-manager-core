use dotenv::dotenv;

use rust_db_manager_core::commons::configuration::configuration::Configuration;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _ = Configuration::initialize();
    println!("rust-db-manager!");
}