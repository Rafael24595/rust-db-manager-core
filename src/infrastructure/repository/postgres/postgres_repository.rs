use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use url::Url;

use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{
        action::{definition::action_definition::ActionDefinition, generate::action::Action}, collection::{collection_data::CollectionData, collection_definition::CollectionDefinition, generate_collection_query::GenerateCollectionQuery}, connection_data::ConnectionData, data_base::generate_database_query::GenerateDatabaseQuery, document::{document_data::DocumentData, document_schema::DocumentSchema}, filter::{
            collection_query::CollectionQuery, data_base_query::DataBaseQuery, definition::filter_definition::FilterDefinition, document_query::DocumentQuery
        }, table::{
            definition::table_definition::TableDefinition, group::table_data_group::TableDataGroup,
        }
    },
    infrastructure::repository::i_db_repository::IDBRepository,
};

pub struct PostgresRepository {
    client: Client,
}

impl PostgresRepository {

    pub async fn new(connection: &ConnectionData) -> Result<Box<dyn IDBRepository>, ConnectException> {
        let client = PostgresRepository::connect(&connection.connection()).await;
        if let Err(err) = client {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let instance = PostgresRepository {
            client: client.ok().unwrap(),
        };

        Ok(Box::new(instance))
    }

    async fn connect(connection: &str) -> Result<Client, ConnectException> {
        let url = Url::parse(connection);
        if let Err(err) = url {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }
    
        let url = url.unwrap();

        if url.scheme() != "postgres" {
            let exception = ConnectException::new(String::from("Invalid scheme. Expected 'postgres'."));
            return Err(exception);
        }

        let username = url.username().to_string();
        let password = url.password().unwrap_or("").to_string();
        let host = url.host_str().unwrap_or("localhost").to_string();
        let port = url.port().unwrap_or(5432);

        let connection = format!("host={} port={} user={} password={}", host, port, username, password);

        let result = tokio_postgres::connect(&connection, NoTls).await;
        if let Err(err) = result {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let (client, connection) = result.unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(client)
    }

}

#[async_trait]
impl IDBRepository for PostgresRepository {

    async fn status(&self) -> Result<(), ConnectException> {
        match self.client.simple_query("SELECT 1").await {
            Ok(_) => Ok(()),
            Err(err) => Err(ConnectException::new(err.to_string())),
        }
    }

    async fn metadata(&self) -> Result<Vec<TableDataGroup>, ConnectException> {
        todo!()
    }

    async fn data_base_metadata(
        &self,
        query: &DataBaseQuery,
    ) -> Result<Vec<TableDataGroup>, ConnectException> {
        todo!()
    }

    async fn data_base_find_all(&self) -> Result<Vec<String>, ConnectException> {
        let rows = self.client
            .query(
                "SELECT datname 
                FROM pg_database 
                WHERE datistemplate = false",
                &[],
            ).await;

        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }
        
        let data_bases = rows.iter()
            .map(|r| r.get(0)
                .map(|r| String::from(r.get::<usize, &str>(0)))
                .unwrap())
            .collect::<Vec<String>>();

        Ok(data_bases)
    }

    async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        todo!()
    }

    async fn data_base_create(
        &self,
        query: &GenerateDatabaseQuery,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn data_base_drop(
        &self,
        query: &GenerateDatabaseQuery,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_accept_schema(&self) -> Result<CollectionDefinition, ConnectException> {
        todo!()
    }

    async fn collection_metadata(
        &self,
        query: &CollectionQuery,
    ) -> Result<Vec<TableDataGroup>, ConnectException> {
        todo!()
    }

    async fn collection_information(
        &self,
        query: &CollectionQuery,
    ) -> Result<Vec<TableDefinition>, ConnectException> {
        todo!()
    }

    async fn collection_actions(
        &self,
        query: &CollectionQuery,
    ) -> Result<Vec<ActionDefinition>, ConnectException> {
        todo!()
    }

    async fn collection_action(
        &self,
        query: &CollectionQuery,
        code: &String,
    ) -> Result<Option<ActionDefinition>, ConnectException> {
        todo!()
    }

    async fn collection_execute_action(
        &self,
        query: &CollectionQuery,
        action: &Action,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_find_all(
        &self,
        query: &DataBaseQuery,
    ) -> Result<Vec<String>, ConnectException> {
        todo!()
    }

    async fn collection_exists(&self, query: &CollectionQuery) -> Result<bool, ConnectException> {
        todo!()
    }

    async fn collection_create(
        &self,
        query: &GenerateCollectionQuery,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_drop(
        &self,
        query: &GenerateCollectionQuery,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_rename(
        &self,
        query: &CollectionQuery,
        name: &str,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_export(
        &self,
        query: &CollectionQuery,
    ) -> Result<Vec<DocumentData>, ConnectException> {
        todo!()
    }

    async fn collection_import(
        &self,
        query: &CollectionQuery,
        documents: Vec<String>,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn filter_schema(&self) -> Result<FilterDefinition, ConnectException> {
        todo!()
    }

    async fn find_all(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        todo!()
    }

    async fn find_query(&self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        todo!()
    }

    async fn find(&self, query: &DocumentQuery) -> Result<Option<DocumentData>, ConnectException> {
        todo!()
    }

    async fn schema(&self, query: &CollectionQuery) -> Result<DocumentSchema, ConnectException> {
        todo!()
    }

    async fn insert(
        &self,
        query: &CollectionQuery,
        value: &str,
    ) -> Result<DocumentData, ConnectException> {
        todo!()
    }

    async fn update(
        &self,
        query: &DocumentQuery,
        value: &str,
    ) -> Result<Vec<DocumentData>, ConnectException> {
        todo!()
    }

    async fn delete(&self, query: &DocumentQuery) -> Result<Vec<DocumentData>, ConnectException> {
        todo!()
    }
    
}
