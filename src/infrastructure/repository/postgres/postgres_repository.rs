use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use tokio_postgres::{Client, NoTls};
use url::Url;

use crate::{
    commons::{configuration::definition::postgres::postgres_collection, exception::connect_exception::ConnectException},
    domain::{
        action::{definition::action_definition::ActionDefinition, generate::action::Action}, collection::{collection_data::CollectionData, collection_definition::CollectionDefinition, collections_reference_definition::CollectionReferenceDefinition, generate_collection_query::GenerateCollectionQuery}, connection_data::ConnectionData, data_base::generate_database_query::GenerateDatabaseQuery, document::{document_data::DocumentData, document_schema::DocumentSchema, e_document_format::EDocumentFormat}, field::generate::{field_data::FieldData, field_reference::FieldReference}, filter::{
            collection_query::CollectionQuery, data_base_query::DataBaseQuery, definition::filter_definition::FilterDefinition, document_query::DocumentQuery
        }, table::{
            definition::table_definition::TableDefinition, group::table_data_group::TableDataGroup
        }
    },
    infrastructure::repository::{i_db_repository::IDBRepository, postgres::postgres_utils::postgres_to_json_type},
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

    async fn connect_table_from_document(&mut self, query: &DocumentQuery) -> Result<&Client, ConnectException> {
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

    async fn total_documents(&mut self, query: &DocumentQuery) -> Result<i64, ConnectException> {
        let client = self.connect_table_from_document(query).await?;

        let query = format!("SELECT COUNT(*) FROM {};", query.collection());
        let rows = client.query(&query, &[]).await;
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let rows = rows.unwrap();
        
        let row = rows.get(0);
        if row.is_none() {
            let exception = ConnectException::new_str("Cannot count table documents.");
            return Err(exception);
        }

        let row = row.unwrap();

        let count: i64 = row.get(0);

        Ok(count)
    }

    async fn keys(&mut self, query: &CollectionQuery) -> Result<(Vec<String>, HashMap<String, FieldReference>), ConnectException> {
        let client = self.connect_table_from_collection(query).await?;
        let rows = client.query(
            "SELECT kcu.column_name,
                    tc.constraint_type AS key_type,
                    ccu.table_name AS foreign_table,
                    ccu.column_name AS foreign_column
             FROM information_schema.key_column_usage kcu
             JOIN information_schema.table_constraints tc
               ON kcu.constraint_name = tc.constraint_name
             LEFT JOIN information_schema.constraint_column_usage ccu
               ON ccu.constraint_name = tc.constraint_name
             WHERE kcu.table_schema = $1
               AND kcu.table_name = $2;",
            &[&"public", &query.collection()]).await;
    
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let mut primary_keys = Vec::new();
        let mut references = HashMap::new();

        for row in rows.unwrap() {
            let column_name: String = row.get("column_name");
            let key_type: Option<String> = row.get("key_type");
            let foreign_table: Option<String> = row.get("foreign_table");
            let foreign_column: Option<String> = row.get("foreign_column");

            if foreign_column.is_none() && foreign_table.is_none() {
                continue;
            }

            let foreing_table = foreign_table.unwrap();
            let foreign_column = foreign_column.unwrap();
            let reference = FieldReference::new(foreing_table.to_owned(), foreign_column.to_owned(), true);

            let key_type = key_type
                .unwrap_or(String::new());

            match key_type.as_str() {
                "PRIMARY KEY" => { primary_keys.push(column_name); },
                "FOREIGN KEY" => { references.insert(column_name, reference); },
                _ => {},
            };
        }

        Ok((primary_keys, references))
    }

    fn make_document_data(&self, data_base: &str, collection: &str, document: &Value) -> Result<DocumentData, ConnectException> {
        let json = serde_json::to_string(&document);
        if let Err(error) = json {
            let exception = ConnectException::new(error.to_string());
            return Err(exception);
        }

        Ok(DocumentData::new(
            EDocumentFormat::TABLE, data_base.to_string(), collection.to_string(), json.ok().unwrap()
        ))
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

    async fn data_base_schema(&mut self, query: &DataBaseQuery) -> Result<CollectionDefinition, ConnectException> {
        let json = postgres_collection();
        let mut definition: CollectionDefinition = serde_json::from_str(&json).expect("Failed to parse JSON");

        let client = self.connect_table_from_db(query).await?;

        let query = "
            SELECT 
                table_name, 
                column_name 
            FROM 
                information_schema.columns
            WHERE 
                table_schema = 'public'
            ORDER BY 
                table_name, ordinal_position;";

        let rows = client.query(query, &[]).await;
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }
        
        let rows = rows.unwrap();

        let mut tables = HashMap::new();

        for row in rows {
            let table_name: &str = row.get("table_name");
            let column_name: &str = row.get("column_name");

            let files = tables.entry(table_name.to_string())
                .or_insert_with(|| CollectionReferenceDefinition::new(table_name.to_string(), Vec::new(), true));
            files.push(column_name);
        }

        let tables = tables.values().cloned().collect();

        definition.push_references(tables);

        Ok(definition)
    }

    async fn data_base_metadata(&mut self, query: &DataBaseQuery) -> Result<Vec<TableDataGroup>, ConnectException> {
        let client = self.connect_table_from_db(query).await?;
        ExtractorMetadataPostgres::from_data_base(client).await
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

    async fn data_base_create(&mut self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let create_db_query = format!("CREATE DATABASE {};", data_base);

        let result = self.client_general.batch_execute(&create_db_query).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(data_base.to_string())
    }

    async fn data_base_drop(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let create_db_query = format!("DROP DATABASE {};", data_base);
        
        let result = self.client_general.batch_execute(&create_db_query).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(data_base.to_string())
    }

    async fn collection_metadata(&mut self, query: &CollectionQuery) -> Result<Vec<TableDataGroup>, ConnectException> {
        let client = self.connect_table_from_collection(query).await?;
        ExtractorMetadataPostgres::from_collection(query, client).await
    }

    async fn collection_information(
        &self,
        query: &CollectionQuery,
    ) -> Result<Vec<TableDefinition>, ConnectException> {
        todo!()
    }

    async fn collection_actions(&mut self, query: &CollectionQuery) -> Result<Vec<ActionDefinition>, ConnectException> {
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

    async fn collection_execute_action(&self, query: &CollectionQuery, action: &Action) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_find_all(&mut self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
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

    async fn collection_exists(&mut self, query: &CollectionQuery) -> Result<bool, ConnectException> {
        todo!()
    }

    async fn collection_create(&mut self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        let client =  self.connect_table(&query.data_base()).await?;

        let create_collection_query = query.to_postgres_query();

        let result = client.batch_execute(&create_collection_query).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(query.collection().to_string())
    }

    async fn collection_drop(&mut self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        let client =  self.connect_table(&query.data_base()).await?;

        let drop_query = format!("DROP TABLE IF EXISTS {}", query.collection());
        let result = client.execute(&drop_query, &[]).await;
        if let Err(result) = result {
            let exception = ConnectException::new(result.to_string());
            return Err(exception);
        }

        Ok(query.collection().to_string())
    }

    async fn collection_rename(
        &self,
        query: &CollectionQuery,
        name: &str,
    ) -> Result<String, ConnectException> {
        todo!()
    }

    async fn collection_export(
        &mut self,
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

    async fn find_all(&mut self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        todo!()
    }

    async fn find_query(&mut self, query: &DocumentQuery) -> Result<CollectionData, ConnectException> {
        let client = self.connect_table_from_document(query).await?;

        let sql = query.as_postres_sql()?;
        let rows = client.query(&sql, &[]).await;
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let total = self.total_documents(&query).await?;

        let mut documents = Vec::new();
        for row in rows.unwrap() {
            let document: Value = row.get("row_to_json");
            let data = self.make_document_data(query.data_base(), query.collection(), &document)?;
            documents.push(data);
        }

        let data = CollectionData::new(
            total as usize,
            query.limit(),
            query.skip(), 
            documents
        );

        Ok(data)
    }

    async fn find(&mut self, query: &DocumentQuery) -> Result<Option<DocumentData>, ConnectException> {
        todo!()
    }

    async fn schema(&mut self, query: &CollectionQuery) -> Result<DocumentSchema, ConnectException> {
        let client = self.connect_table_from_collection(query).await?;
        let rows = client.query(
            "SELECT column_name, data_type, character_maximum_length
             FROM information_schema.columns
             WHERE table_schema = 'public' AND table_name = $1",
            &[&query.collection()]).await;
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let keys = self.keys(query).await;
        if let Err(err) = keys {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let (primary_keys, references) = keys.unwrap();

        let mut fields = Vec::new();
        for (i, row) in rows.unwrap().iter().enumerate() {
            let column_name: &str = row.get("column_name");
            let data_type: &str = row.get("data_type");
            let char_max_len: Option<i32> = row.get("character_maximum_length");

            let column_name = String::from(column_name);

            let size = match char_max_len {
                Some(s) => s,
                None => 0
            };

            let primary_key = primary_keys.contains(&column_name);
            let reference = match references.get(&column_name) {
                Some(s) => vec![s.clone()],
                None => Vec::new()
            };

            let json_type = postgres_to_json_type(data_type);

            let field = FieldData::new(
                i as i32, 
                column_name.to_uppercase(), 
                column_name, 
                primary_key, 
                char_max_len.is_some(), 
                size, 
                true, 
                json_type, 
                Vec::new(), 
                reference
            );

            fields.push(field);
        }

        let schema = DocumentSchema::new(Vec::new(), true, true, fields);

        Ok(schema)
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
