mod commons {
    pub mod exception {
        pub mod connect_exception;
    }
}
mod infrastructure {
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
}
mod service {}

fn main() {
    println!("rust-db-manager!");
}