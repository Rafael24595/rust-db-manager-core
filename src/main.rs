use crate::domain::filter::filter_element::FilterElement;

mod commons {
    pub mod exception {
        pub mod connect_exception;
    }
    pub mod utils;
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
    pub mod connection_data;
}
mod service {}

fn main() {

    let base = FilterElement::new()
        .push(FilterElement::from_i16(String::from("key"), 3).negate_ref())
        .push(FilterElement::from_bool(String::from("status"), true))
        .push(FilterElement::from_query(String::from("{ \"$match\": { \"$and\": [ { \"key\": 2 } ] } }")));

    let result = base.as_mongo_agregate();

    if result.is_err() {
        println!("{}", result.err().unwrap());    
    }

    println!("rust-db-manager!");
}