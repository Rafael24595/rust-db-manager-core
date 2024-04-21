pub mod commons {
    pub mod configuration {
        pub mod configuration;
    }
    pub mod exception {
        pub mod connect_exception;
    }
    pub mod utils;
}
pub mod infrastructure {
    pub mod repository {
        pub mod db_dictionary;
        pub mod e_db_repository;
        pub mod i_db_repository;
        pub mod mongo_db_repository;
    }
    pub mod db_service_lite;
    pub mod db_service;
}
pub mod domain {
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
pub mod service {
    pub mod service;
}