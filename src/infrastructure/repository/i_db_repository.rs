use async_trait::async_trait;

use crate::{commons::exception::connect_exception::ConnectException, domain::{data_base_info::DataBaseInfo, filter::data_base_query::DataBaseQuery, generate::{generate_collection_query::GenerateCollectionQuery, generate_data_base_query::GenerateDataBaseQuery}}};

#[async_trait]
pub trait IDBRepository: Clone + Send + Sync {
    //TODO: Replace bytes vector returns with specific entities.
    async fn status(&self) -> Result<(), ConnectException>;
    async fn info(&self) -> DataBaseInfo;
    async fn data_base_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException>;
    async fn create_data_base(&self, query: GenerateDataBaseQuery) -> String;
    async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException>;
    async fn collection_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException>;
    async fn create_collection(&self, query: GenerateCollectionQuery) -> String;
    async fn list_collections(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find(&self, query: DataBaseQuery) -> Result<Option<String>, ConnectException>;
    async fn find_query_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_query(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_all_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn find_all(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException>;
    async fn insert(&self, query: DataBaseQuery, value: String) -> Result<String,ConnectException>;
    async fn update(&self, query: DataBaseQuery, value: String) -> Vec<u8>;
    async fn delete(&self, query: DataBaseQuery) -> Result<Vec<String>,ConnectException>;
}