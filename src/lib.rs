pub mod commons {
    pub mod configuration {
        pub mod definition {
            pub mod mongo_db;
        }
        pub mod configuration;
    }
    pub mod exception {
        pub mod configuration_exception;
        pub mod connect_exception;
    }
    pub mod utils;
}
pub mod infrastructure {
    pub mod repository {
        pub mod mongo_db {
            pub mod e_action;
            pub mod e_filter_attributes;
            pub mod extractor_metadata_mongo_db;
            pub mod mongo_db_actions;
            pub mod mongo_db_repository;
            pub mod mongo_utils;
        }
        pub mod db_dictionary;
        pub mod e_db_repository;
        pub mod i_db_repository;
    }
    pub mod db_service_lite;
    pub mod db_service;
}
pub mod domain {
    pub mod action {
        pub mod definition {
            pub mod action_form_collection;
            pub mod action_definition;
            pub mod action_form;
            pub mod form_default;
            pub mod form_field_definition;
        }
        pub mod generate {
            pub mod action_form;
            pub mod action;
            pub mod form_field;
        }
    }
    pub mod collection {
        pub mod collection_data;
        pub mod collection_definition;
        pub mod generate_collection_query;
    }
    pub mod data_base {
        pub mod generate_database_query;
    }
    pub mod document {
        pub mod document_data;
        pub mod document_key;
        pub mod document_key_attribute;
        pub mod document_schema;
    }
    pub mod field {
        pub mod definition {
            pub mod field_attribute_default_definition;
            pub mod field_attribute_definition;
            pub mod field_definition;
        }
        pub mod generate {
            pub mod field_attribute;
            pub mod field_data;
            pub mod field_reference;
        }
        pub mod e_field_code;
    }
    pub mod filter {
        pub mod definition {
            pub mod filter_attribute_default_definition;
            pub mod filter_attribute_definition;
            pub mod filter_definition;
        }
        pub mod collection_query;
        pub mod data_base_query;
        pub mod document_query;
        pub mod e_filter_category;
        pub mod filter_value;
        pub mod filter_value_attribute;
        pub mod filter_element;
    }
    pub mod table {
        pub mod definition {
            pub mod table_definition;
            pub mod table_field_definition;
            pub mod table_row_definition;
        }
        pub mod group {
            pub mod table_data_field;
            pub mod table_data_group;
        }
    }
    pub mod e_json_type;
    pub mod connection_data;
}
pub mod service {
    pub mod service;
}