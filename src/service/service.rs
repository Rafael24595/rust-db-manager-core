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
    infrastructure::repository::i_db_repository::IDBRepository,
};

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

    pub async fn metadata(&self) -> Result<Vec<TableDataGroup>, ConnectException> {
        return self.repository.metadata().await;
    }

    pub async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        return self.repository.data_base_exists(query).await;
    }

    pub async fn data_base_create(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        return self.repository.data_base_create(query).await;
    }

    pub async fn data_base_drop(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        return self.repository.data_base_drop(query).await;
    }

    pub async fn data_base_find_all(&self) -> Result<Vec<String>, ConnectException> {
        return self.repository.data_base_find_all().await;
    }

    pub async fn data_base_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException> {
        return self.repository.data_base_metadata(query).await;
    }

    pub async fn collection_accept_schema(&self) -> Result<CollectionDefinition, ConnectException> {
        return self.repository.collection_accept_schema().await;
    }

    pub async fn collection_metadata(&self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException> {
        return self.repository.collection_metadata(query).await;
    }

    pub async fn collection_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        return self.repository.collection_exists(query).await;
    }

    pub async fn collection_create(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.collection_create(query).await;
    }

    pub async fn collection_drop(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.collection_drop(query).await;
    }

    pub async fn collection_find_all(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.collection_find_all(query).await;
    }

    pub async fn find_query_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_query_lite(query).await;
    }

    pub async fn find_query(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException> {
        return self.repository.find_query(query).await;
    }

    pub async fn find_all_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.find_all_lite(query).await;
    }

    pub async fn find_all(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>, ConnectException> {
        return self.repository.find_all(query).await;
    }
    
    pub async fn find(&self, query: &DataBaseQuery) -> Result<Option<DocumentData>, ConnectException> {
        return self.repository.find(query).await;
    }

    pub async fn schema(&self, query: &DataBaseQuery) ->  Result<DocumentSchema, ConnectException> {
        return self.repository.schema(query).await;
    }

    pub async fn insert(&self, query: &DataBaseQuery, value: &str) -> Result<DocumentData, ConnectException> {
        return self.repository.insert(query, &value).await;
    }

    pub async fn update(&self, query: &DataBaseQuery, value: &str) -> Result<Vec<DocumentData>, ConnectException> {
        return self.repository.update(query, value).await;
    }

    pub async fn delete(&self, query: &DataBaseQuery) -> Result<Vec<DocumentData>,ConnectException> {
        return self.repository.delete(query).await;
    }

}