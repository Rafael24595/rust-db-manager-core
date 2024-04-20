use crate::{domain::connection_data::ConnectionData, infrastructure::{manager::terminal::data_base::manager_database::ManagerDatabase, repository::mongo_db_repository::MongoDbRepository}, service::service::Service};

mod commons {
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
        pub mod i_db_repository;
        pub mod mongo_db_repository;
    }
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
    let data = ConnectionData::new(String::from("mongodb://root:example@localhost:27017"));
    let repository = MongoDbRepository::new(data).await.ok().unwrap();
    let service = Service::from(repository);

    let mut terminal = ManagerDatabase::new(service);
    terminal.launch().await;

    println!("rust-db-manager!");
}