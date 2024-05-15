use async_trait::async_trait;

use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{
        collection::{
            collection_definition::CollectionDefinition,
            generate_collection_query::GenerateCollectionQuery,
        },
        data_base::generate_database_query::GenerateDatabaseQuery,
        document::{document_data::DocumentData, document_schema::DocumentSchema},
        filter::data_base_query::DataBaseQuery,
        table::table_data_group::TableDataGroup,
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
    async fn collection_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException>;
    async fn collection_find_all(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn collection_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException>;
    async fn collection_create(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn collection_drop(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException>;
    async fn collection_rename(&self, query: &DataBaseQuery, name: &str) -> Result<String, ConnectException>;
    async fn collection_export(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;
    async fn collection_import(&self, query: &DataBaseQuery, documents: Vec<String>) -> Result<String, ConnectException>;

    async fn find_query_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_query(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;

    async fn find_all_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_all(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;
    async fn find(&self, query: &DataBaseQuery) -> Result<Option<DocumentData>, ConnectException>;
    async fn schema(&self, query: &DataBaseQuery) -> Result<DocumentSchema, ConnectException>;
    async fn insert(&self, query: &DataBaseQuery, value: &str) -> Result<DocumentData, ConnectException>;
    async fn update(&self, query: &DataBaseQuery, value: &str) -> Result<Vec<DocumentData>, ConnectException>;
    async fn delete(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException>;
}