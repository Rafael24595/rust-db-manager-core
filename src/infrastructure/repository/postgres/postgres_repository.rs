use std::collections::HashMap;

use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use url::Url;

use crate::{
    commons::{configuration::definition::postgres::postgres_collection, exception::connect_exception::ConnectException},
    domain::{
        action::{definition::action_definition::ActionDefinition, generate::action::Action}, collection::{collection_data::CollectionData, collection_definition::CollectionDefinition, generate_collection_query::GenerateCollectionQuery}, connection_data::ConnectionData, data_base::{self, generate_database_query::GenerateDatabaseQuery}, document::{document_data::DocumentData, document_schema::DocumentSchema}, filter::{
            collection_query::CollectionQuery, data_base_query::DataBaseQuery, definition::filter_definition::FilterDefinition, document_query::DocumentQuery
        }, table::{
            definition::table_definition::TableDefinition, group::table_data_group::TableDataGroup,
        }
    },
    infrastructure::repository::i_db_repository::IDBRepository,
};

use super::extractor_metadata_postgres::ExtractorMetadataPostgres;

pub struct PostgresRepository {
    connection: String,
    client_general: Client,
    client_collections: HashMap<String, Client>
}

impl PostgresRepository {

    pub async fn new(connection: &ConnectionData) -> Result<Box<dyn IDBRepository>, ConnectException> {
        let result = PostgresRepository::connect(&connection.connection()).await;
        if let Err(err) = result {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let (connection, client) = result.unwrap();

        let instance: PostgresRepository = PostgresRepository {
            connection: connection,
            client_general: client,
            client_collections: HashMap::new()
        };

        Ok(Box::new(instance))
    }

    async fn connect(connection: &str) -> Result<(String, Client), ConnectException> {
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

        let connection_string = format!("host={} port={} user={} password={}", host, port, username, password);

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

        Ok((connection_string, client))
    }

    async fn connect_table_from_db(&mut self, query: &DataBaseQuery) -> Result<&Client, ConnectException> {
        self.connect_table(&query.data_base()).await
    }
    
    async fn connect_table_from_collection(&mut self, query: &CollectionQuery) -> Result<&Client, ConnectException> {
        self.connect_table(&query.data_base()).await
    }

    async fn connect_table(&mut self, data_base: &str) -> Result<&Client, ConnectException> {
        if self.client_collections.contains_key(data_base) {
            let client = self.client_collections.get(data_base).unwrap();
            return Ok(client);
        }

        let connection = format!("{} dbname={}", self.connection, data_base);

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

        self.client_collections.insert(data_base.to_string(), client);

        let client = self.client_collections.get(data_base).unwrap();
        Ok(client)
    }

}

#[async_trait]
impl IDBRepository for PostgresRepository {

    async fn status(&self) -> Result<(), ConnectException> {
        match self.client_general.simple_query("SELECT 1").await {
            Ok(_) => Ok(()),
            Err(err) => Err(ConnectException::new(err.to_string())),
        }
    }

    async fn metadata(&self) -> Result<Vec<TableDataGroup>, ConnectException> {
        ExtractorMetadataPostgres::from_db(&self.client_general).await
    }

    async fn data_base_metadata(
        &mut self,
        query: &DataBaseQuery,
    ) -> Result<Vec<TableDataGroup>, ConnectException> {
        let client = self.connect_table_from_db(query).await?;
        ExtractorMetadataPostgres::from_collection(client).await
    }

    async fn data_base_find_all(&self) -> Result<Vec<String>, ConnectException> {
        let rows = self.client_general
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
        
        let rows = rows.unwrap();

        let data_bases = rows.iter()
            .map(|r| String::from(r.get::<usize, &str>(0)))
            .collect::<Vec<String>>();

        Ok(data_bases)
    }

    async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        let check_db_query = format!(
            "SELECT 1 FROM pg_database WHERE datname = '{}';",
            query.data_base()
        );
        
        let db_exists = match self.client_general.query_one(&check_db_query, &[]).await {
            Ok(_) => true,
            Err(_) => false,
        };

        Ok(db_exists)
    }

    async fn data_base_create(
        &self,
        query: &GenerateDatabaseQuery,
    ) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let create_db_query = format!("CREATE DATABASE {};", data_base);

        let result = self.client_general.batch_execute(&create_db_query).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(data_base)
    }

    async fn data_base_drop(
        &self,
        query: &GenerateDatabaseQuery,
    ) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let create_db_query = format!("DROP DATABASE {};", data_base);
        
        let result = self.client_general.batch_execute(&create_db_query).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(data_base)
    }

    async fn collection_accept_schema(&self) -> Result<CollectionDefinition, ConnectException> {
        let json = postgres_collection();
        let definition: CollectionDefinition = serde_json::from_str(&json).expect("Failed to parse JSON");
        Ok(definition)
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
        &mut self,
        query: &CollectionQuery,
    ) -> Result<Vec<ActionDefinition>, ConnectException> {
        let client = self.connect_table_from_collection(query).await?;
        let definition = ExtractorMetadataPostgres::collection_actions(client).await?;
        Ok(definition)
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
        &mut self,
        query: &DataBaseQuery,
    ) -> Result<Vec<String>, ConnectException> {
        let client = self.connect_table_from_db(query).await?;

        let rows = client.query("
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_type = 'BASE TABLE';", &[]).await;

        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }
        
        let rows = rows.unwrap();

        let tables = rows.iter()
            .map(|r| r.get::<usize, &str>(0).to_string())
            .collect::<Vec<String>>();

        Ok(tables)
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
