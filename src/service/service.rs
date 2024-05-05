use crate::{commons::exception::connect_exception::ConnectException, domain::{data_base_group_data::DataBaseDataGroup, filter::data_base_query::DataBaseQuery, generate::{generate_collection_query::GenerateCollectionQuery, generate_database_query::GenerateDatabaseQuery}}, infrastructure::repository::i_db_repository::IDBRepository};

#[derive(Clone)]
pub struct Service<T: IDBRepository> {
    repository: T,
}

impl <T: IDBRepository> Service<T> {

    pub fn from(repository: T) -> Service<T> {
        Service { repository }
    }

    pub async fn status(&self) -> Result<(), ConnectException> {
        return self.repository.status().await;
    }

    pub async fn metadata(&self) -> Result<Vec<DataBaseDataGroup>, ConnectException> {
        return self.repository.metadata().await;
    }

    pub async fn data_base_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException> {
        return self.repository.data_base_exists(query).await;
    }

    pub async fn create_data_base(&self, query: GenerateDatabaseQuery) -> Result<String, ConnectException> {
        return self.repository.create_data_base(query).await;
    }

    pub async fn drop_data_base(&self, query: GenerateDatabaseQuery) -> Result<String, ConnectException> {
        return self.repository.drop_data_base(query).await;
    }

    pub async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_data_bases().await;
    }

    pub async fn collection_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException> {
        return self.repository.collection_exists(query).await;
    }

    pub async fn create_collection(&self, query: GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.create_collection(query).await;
    }

    pub async fn drop_collection(&self, query: GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.drop_collection(query).await;
    }

    pub async fn list_collections(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.list_collections(query).await;
    }

    pub async fn find(&self, query: DataBaseQuery) -> Result<Option<String>, ConnectException> {
        return self.repository.find(query).await;
    }

    pub async fn find_query_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_query_lite(query).await;
    }

    pub async fn find_query(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_query(query).await;
    }

    pub async fn find_all_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_all_lite(query).await;
    }

    pub async fn find_all(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_all(query).await;
    }

    pub async fn insert(&self, query: DataBaseQuery, value:String) -> Result<String,ConnectException>{
        return self.repository.insert(query, value).await;
    }

    pub async fn delete(&self, query: DataBaseQuery) -> Result<Vec<String>,ConnectException>{
        return self.repository.delete(query).await;
    }

}