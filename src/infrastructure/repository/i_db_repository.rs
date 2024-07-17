use async_trait::async_trait;

use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{
        action::{definition::action_definition::ActionDefinition, generate::action::Action}, collection::{
            collection_data::CollectionData, collection_definition::CollectionDefinition,
            generate_collection_query::GenerateCollectionQuery,
        }, data_base::generate_database_query::GenerateDatabaseQuery, document::{document_data::DocumentData, document_schema::DocumentSchema}, filter::{
            collection_query::CollectionQuery, data_base_query::DataBaseQuery,
            definition::filter_definition::FilterDefinition, document_query::DocumentQuery,
        }, table::{definition::table_definition::TableDefinition, group::table_data_group::TableDataGroup}
    },
};

#[async_trait]
pub trait IDBRepository: Clone + Send + Sync {
    async fn status(&self) -> Result<(), ConnectException>;
    async fn metadata(&self) -> Result<Vec<TableDataGroup>, ConnectException>;

    async fn data_base_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn data_base_find_all(&self) -> Result<Vec<String>, ConnectException>;
    async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException>;
    async fn data_base_create(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException>;
    async fn data_base_drop(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException>;

    async fn collection_accept_schema(&self) -> Result<CollectionDefinition, ConnectException>;
    async fn collection_metadata(&self, query: &CollectionQuery) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn collection_information(&self, query: &CollectionQuery) -> Result<Vec<TableDefinition>, ConnectException>;
    async fn collection_actions(&self, query: &CollectionQuery) -> Result<Vec<ActionDefinition>, ConnectException>;
    async fn collection_action(&self, query: &CollectionQuery, code: &String) -> Result<Option<ActionDefinition>, ConnectException>;
    async fn collection_execute_action(&self, query: &CollectionQuery, action: &Action) -> Result<String, ConnectException>;
    async fn collection_find_all(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn collection_exists(&self, query: &CollectionQuery) -> Result<bool, ConnectException>;
    async fn collection_create(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn collection_drop(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn collection_rename(&self, query: &CollectionQuery, name: &str) -> Result<String, ConnectException>;
    async fn collection_export(&self, query: &CollectionQuery) -> Result<Vec<DocumentData>, ConnectException>;
    async fn collection_import(&self, query: &CollectionQuery, documents: Vec<String>) -> Result<String, ConnectException>;
    
    async fn filter_schema(&self) -> Result<FilterDefinition, ConnectException>;
    async fn find_all(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException>;
    async fn find_query(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException>;
    async fn find(&self, query: &DocumentQuery) -> Result<Option<DocumentData>, ConnectException>;
    async fn schema(&self, query: &CollectionQuery) -> Result<DocumentSchema, ConnectException>;
    async fn insert(&self, query: &CollectionQuery, value: &str) -> Result<DocumentData, ConnectException>;
    async fn update(&self, query: &DocumentQuery, value: &str) -> Result<Vec<DocumentData>, ConnectException>;
    async fn delete(&self, query: &DocumentQuery) -> Result<Vec<DocumentData>, ConnectException>;
}