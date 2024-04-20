use crate::{commons::configuration::configuration::Configuration, domain::connection_data::ConnectionData, infrastructure::{db_service::DBService, manager::terminal::data_base::manager_database::ManagerDatabase, repository::{e_db_repository::EDBRepository, mongo_db_repository::MongoDbRepository}}, service::service::Service};

mod commons {
    pub mod configuration {
        pub mod configuration;
    }
    pub mod exception {
        pub mod connect_exception;
    }
    pub mod utils;
}
mod infrastructure {
    pub mod manager {
        pub mod terminal {
            pub mod data_base {
                pub mod manager_database;
                pub mod path_interpeter;
                pub mod utils;
            }
            pub mod i_manager;
            pub mod terminal_option;
            pub mod terminal_cursor;
            pub mod terminal_manager;
        }
    }
    pub mod repository {
        pub mod db_dictionary;
        pub mod e_db_repository;
        pub mod i_db_repository;
        pub mod mongo_db_repository;
    }
    pub mod db_service;
}
mod domain {
    pub mod filter {
        pub mod data_base_query;
        pub mod e_filter_category;
        pub mod filter_value;
        pub mod filter_element;
    }
    pub mod generate {
        pub mod e_collection_field;
        pub mod collection_field;
        pub mod generate_database_query;
        pub mod generate_collection_query;
    }
    pub mod connection_data;
    pub mod data_base_info;
}
mod service {
    pub mod service;
}

#[tokio::main]
async fn main() {
    let _ = Configuration::initialize();

    let key = String::from("MONGO_DB");
    let data = ConnectionData::new(EDBRepository::MongoDB, String::from("mongodb://root:example@localhost:27017"));
    let serv = DBService::new(key.clone(), String::from("ADMIN"), data);

    Configuration::push_service(key.clone(), serv);

    let serv = Configuration::find_service(key).unwrap();
    let service = serv.instance().await.expect("Initialize error.");

    let mut terminal = ManagerDatabase::new(service);
    terminal.launch().await;

    println!("rust-db-manager!");
}