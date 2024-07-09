use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{
        action::{definition::action_definition::ActionDefinition, generate::action::Action},
        collection::{
            collection_data::CollectionData, collection_definition::CollectionDefinition,
            generate_collection_query::GenerateCollectionQuery,
        },
        data_base::generate_database_query::GenerateDatabaseQuery,
        document::{document_data::DocumentData, document_schema::DocumentSchema},
        filter::{
            collection_query::CollectionQuery, data_base_query::DataBaseQuery,
            definition::filter_definition::FilterDefinition, document_query::DocumentQuery,
        },
        table::{
            definition::table_definition::TableDefinition, group::table_data_group::TableDataGroup,
        },
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

    pub async fn collection_metadata(&self, query: &CollectionQuery) -> Result<Vec<TableDataGroup>, ConnectException> {
        return self.repository.collection_metadata(query).await;
    }

    pub async fn collection_information(&self, query: &CollectionQuery) -> Result<Vec<TableDefinition>, ConnectException> {
        return self.repository.collection_information(query).await;
    }

    pub async fn collection_actions(&self, query: &CollectionQuery) -> Result<Vec<ActionDefinition>, ConnectException> {
        return self.repository.collection_actions(query).await;
    }

    pub async fn collection_action(&self, query: &CollectionQuery, code: &String) -> Result<Option<ActionDefinition>, ConnectException> {
        return self.repository.collection_action(query, code).await;
    }

    pub async fn collection_execute_action(&self, query: &CollectionQuery, action: &Action) -> Result<String, ConnectException> {
        return self.repository.collection_execute_action(query, action).await;
    }

    pub async fn collection_exists(&self, query: &CollectionQuery) -> Result<bool, ConnectException> {
        return self.repository.collection_exists(query).await;
    }

    pub async fn collection_create(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.collection_create(query).await;
    }

    pub async fn collection_drop(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        return self.repository.collection_drop(query).await;
    }

    pub async fn collection_rename(&self, query: &CollectionQuery, name: &str) -> Result<String, ConnectException> {
        return self.repository.collection_rename(query, name).await;
    }

    pub async fn collection_export(&self, query: &CollectionQuery) -> Result<Vec<DocumentData>, ConnectException> {
        return self.repository.collection_export(query).await;
    }

    pub async fn collection_import(&self, query: &CollectionQuery, documents: Vec<String>) -> Result<String, ConnectException> {
        return self.repository.collection_import(query, documents).await;
    }

    pub async fn collection_find_all(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.repository.collection_find_all(query).await;
    }

    pub async fn filter_schema(&self) -> Result<FilterDefinition, ConnectException> {
        return self.repository.filter_schema().await;
    }

    pub async fn find_query(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        return self.repository.find_query(query).await;
    }

    pub async fn find_all(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        return self.repository.find_all(query).await;
    }
    
    pub async fn find(&self, query: &DocumentQuery) -> Result<Option<DocumentData>, ConnectException> {
        return self.repository.find(query).await;
    }

    pub async fn schema(&self, query: &CollectionQuery) ->  Result<DocumentSchema, ConnectException> {
        return self.repository.schema(query).await;
    }

    pub async fn insert(&self, query: &CollectionQuery, value: &str) -> Result<DocumentData, ConnectException> {
        return self.repository.insert(query, &value).await;
    }

    pub async fn update(&self, query: &DocumentQuery, value: &str) -> Result<Vec<DocumentData>, ConnectException> {
        return self.repository.update(query, value).await;
    }

    pub async fn delete(&self, query: &DocumentQuery) -> Result<Vec<DocumentData>,ConnectException> {
        return self.repository.delete(query).await;
    }

}