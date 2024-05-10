use async_trait::async_trait;

use crate::{commons::exception::connect_exception::ConnectException, domain::{collection::{collection_definition::CollectionDefinition, generate_collection_query::GenerateCollectionQuery}, data_base::generate_database_query::GenerateDatabaseQuery, document::document_data::DocumentData, filter::data_base_query::DataBaseQuery, table::table_data_group::TableDataGroup}};

#[async_trait]
pub trait IDBRepository: Clone + Send + Sync {
    //TODO: Replace bytes vector returns with specific entities.
    async fn status(&self) -> Result<(), ConnectException>;
    async fn data_base_metadata(&self) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException>;
    async fn create_data_base(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException>;
    async fn drop_data_base(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException>;
    async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException>;
    async fn data_base_collections_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn collection_accept_definition(&self) -> Result<CollectionDefinition, ConnectException>;
    async fn collection_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn collection_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException>;
    async fn create_collection(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn drop_collection(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn list_collections(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find(&self, query: &DataBaseQuery) -> Result<Option<DocumentData>, ConnectException>;
    async fn find_query_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_query(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;
    async fn find_all_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_all(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;
    async fn insert(&self, query: &DataBaseQuery, value: String) -> Result<String,ConnectException>;
    async fn update(&self, query: &DataBaseQuery, value: String) -> Vec<u8>;
    async fn delete(&self, query: &DataBaseQuery) -> Result<Vec<String>,ConnectException>;
}