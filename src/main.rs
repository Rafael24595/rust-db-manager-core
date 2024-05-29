use rust_db_manager_core::commons::configuration::configuration::Configuration;

#[tokio::main]
async fn main() {
    let _ = Configuration::initialize();
    println!("rust-db-manager!");
}