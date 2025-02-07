use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tokio_postgres::Client;

use crate::{
    commons::{
        configuration::definition::postgres::postgres_collection_actions,
        exception::connect_exception::ConnectException,
    },
    domain::{
        action::definition::action_definition::ActionDefinition, filter::collection_query::CollectionQuery, table::{definition::{table_definition::TableDefinition, table_row_definition::TableRowDefinition}, group::{e_data_type::EDataType, table_data_group::TableDataGroup}}
    },
};
pub(crate) struct ExtractorMetadataPostgres {
}

impl ExtractorMetadataPostgres {

    pub(crate) async fn from_db(client: &Client) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        
        metadata.push(Self::metadata_general(client).await?);
        metadata.push(Self::metadata_connection(client).await?);

        Ok(metadata)
    }
    
    async fn metadata_general(client: &Client) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("general"));

        group.push(
            String::from("Listen Addresses"),
           Self::listen_addresses(client).await?
        );

        group.push(
            String::from("Port"),
           Self::port(client).await?
        );

        group.push(
            String::from("Version"),
            Self::version(client).await?
        );

        group.push_typed(
            String::from("Uptime"),
           Self::timeup(client).await?,
           EDataType::TIMESTAMP
        );

        group.push_typed(
            String::from("Size"),
           Self::all_size(client).await?,
           EDataType::BYTE
        );

        Ok(group)
    }

    async fn metadata_connection(client: &Client) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(1, String::from("connection"));

        group.push(
            String::from("User"),
           Self::user(client).await?
        );

        group.push(
            String::from("Active Connections"),
           Self::connections_active(client).await?
        );

        group.push(
            String::from("Max Connections"),
           Self::connections_max(client).await?
        );

        Ok(group)
    }

    async fn version(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("SELECT version();", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let version = row.get::<usize, &str>(0);
        Ok(String::from(version))
    }

    async fn timeup(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("SELECT pg_postmaster_start_time();", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let timestamp = row.get::<usize, DateTime<Utc>>(0).timestamp_millis();
        Ok(timestamp.to_string())
    }

    async fn connections_active(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("SELECT COUNT(*) FROM pg_stat_activity;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let connections = row.get::<usize, i64>(0);
        Ok(connections.to_string())
    }

    async fn connections_max(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("SHOW max_connections;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let connections = row.get::<usize, &str>(0);
        Ok(connections.to_string())
    }

    async fn all_size(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("
            SELECT SUM(pg_database_size(datname)) AS total_size_in_bytes
            FROM pg_database
            WHERE datistemplate = false;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let bytes = row.get::<usize, Decimal>(0);

        Ok(bytes.to_string())
    }

    async fn user(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("SELECT current_user;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let user = row.get::<usize, &str>(0);

        Ok(user.to_string())
    }

    async fn listen_addresses(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one(" SHOW listen_addresses;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let listen_addresses = row.get::<usize, &str>(0);

        Ok(listen_addresses.to_string())
    }

    async fn port(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one(" SHOW port;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let port = row.get::<usize, &str>(0);

        Ok(port.to_string())
    }

    pub(crate) async fn from_data_base(client: &Client) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        
        metadata.push(Self::database_metadata_general(client).await?);

        Ok(metadata)
    }

    async fn database_metadata_general(client: &Client) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("general"));

        group.push_typed(
            String::from("Size"),
           Self::data_base_all_size(client).await?,
           EDataType::BYTE
        );

        group.push(
            String::from("Schemas"),
           Self::data_base_count_schemas(client).await?
        );

        Ok(group)
    }

    async fn data_base_all_size(client: &Client) -> Result<String, ConnectException> {
        let row = client.query_one("
            SELECT SUM(pg_total_relation_size(relid)) AS total_size_in_bytes
            FROM pg_catalog.pg_statio_user_tables;", &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let bytes = row.try_get::<usize, Decimal>(0);
        if bytes.is_err() {
            return Ok(0.to_string());
        }

        Ok(bytes.unwrap().to_string())
    }

    async fn data_base_count_schemas(client: &Client) -> Result<String, ConnectException> {
        let row = client.query("
            SELECT schema_name FROM information_schema.schemata;", &[]).await;
        if let Err(err) = row {
            println!("{:?}", err.to_string());
            return Ok(0.to_string());
        }

        let row = row.unwrap();

        Ok(row.len().to_string())
    }

    pub(crate) async fn collection_actions(client: &Client) -> Result<Vec<ActionDefinition>, ConnectException> {
        let json = postgres_collection_actions();
        let definition: Vec<ActionDefinition> = serde_json::from_str(&json).expect("Failed to parse JSON");
    
        Ok(definition)
    }

    pub(crate) async fn from_collection(query: &CollectionQuery, client: &Client) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        
        metadata.push(Self::collection_metadata_general(query, client).await?);

        Ok(metadata)
    }

    async fn collection_metadata_general(query: &CollectionQuery, client: &Client) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("general"));

        let query = format!("
            SELECT 
                pg_table_size(oid) AS table_size,
                pg_total_relation_size(oid) AS total_size,
                pg_indexes_size(oid) AS index_size
            FROM pg_class
            WHERE relkind = 'r'
            AND relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = 'public' AND relname = '{}')
            ORDER BY total_size DESC;", query.collection());

        let row = client.query_one(&query, &[]).await;
        if let Err(err) = row {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let row = row.unwrap();

        let table_size: i64 = row.get("table_size");
        group.push_typed(
            String::from("Table size"),
            table_size.to_string(),
           EDataType::BYTE
        );

        let index_size: i64 = row.get("index_size");
        group.push_typed(
            String::from("Index size"),
            index_size.to_string(),
           EDataType::BYTE
        );

        let total_size: i64 = row.get("total_size");
        group.push_typed(
            String::from("Size"),
            total_size.to_string(),
           EDataType::BYTE
        );

        Ok(group)
    }

    pub(crate) async fn from_keys(query: &CollectionQuery, client: &Client) -> Result<Vec<TableDefinition>, ConnectException> {
        let mut primary_keys = TableDefinition::new(String::from("Primary Keys"));

        let mut primary_titles = TableRowDefinition::new();
        primary_titles.push_title(String::from("Name"));
        primary_titles.push_title(String::from("Type"));

        let mut foreign_keys = TableDefinition::new(String::from("Foreign Keys"));

        let mut foreign_titles = TableRowDefinition::new();
        foreign_titles.push_title(String::from("Table"));
        foreign_titles.push_title(String::from("Column"));
        foreign_titles.push_title(String::from("Type"));

        let rows = client.query(
    "SELECT
                    kcu.column_name,
                    c.data_type,
                    c.character_maximum_length,
                    c.numeric_precision,
                    c.numeric_scale,
                    tc.constraint_type AS key_type,
                    ccu.table_name AS foreign_table,
                    ccu.column_name AS foreign_column
                FROM
                    information_schema.key_column_usage kcu
                JOIN
                    information_schema.table_constraints tc ON kcu.constraint_name = tc.constraint_name
                LEFT JOIN
                    information_schema.constraint_column_usage ccu ON ccu.constraint_name = tc.constraint_name
                JOIN  -- Important: Join with columns table
                    information_schema.columns c ON kcu.table_schema = c.table_schema
                                            AND kcu.table_name = c.table_name
                                            AND kcu.column_name = c.column_name
                WHERE
                    kcu.table_schema = $1
                    AND kcu.table_name = $2;",
            &[&"public", &query.collection()]).await;
        if let Err(err) = rows {
            let exception = ConnectException::new(err.to_string());
            return Err(exception);
        }

        let mut primary_rows = Vec::new();
        let mut foreign_rows = Vec::new();
        for row in rows.unwrap() {
            let column_name: String = row.get("column_name");
            let data_type: String = row.get("data_type");
            let character_maximum_length: Option<i32> = row.get("character_maximum_length");
            let key_type: Option<String> = row.get("key_type");
            let foreign_table: Option<String> = row.get("foreign_table");
            let foreign_column: Option<String> = row.get("foreign_column");

            if foreign_column.is_none() && foreign_table.is_none() {
                continue;
            }
    
            let key_type = key_type
                .unwrap_or(String::new());

            let mut data_type_format = data_type;
            if let Some(character_maximum_length) = character_maximum_length {
                data_type_format = format!("{} ({})", data_type_format, character_maximum_length);
            }

            let mut row = TableRowDefinition::new();

            if key_type.as_str() == "PRIMARY KEY" {
                row.push(column_name);
                row.push(data_type_format);
                primary_rows.push(row);
                continue;
            }

            let foreing_table = foreign_table.unwrap();
            let foreign_column = foreign_column.unwrap();

            row.push(foreing_table.to_owned());
            row.push(foreign_column.to_owned());
            row.push(data_type_format);
            foreign_rows.push(row);
        }

        if primary_rows.len() > 0 {
            primary_keys.push(primary_titles);
            primary_rows.iter().for_each(|r| {
                primary_keys.push(r.clone());
            });
        }

        if foreign_rows.len() > 0 {
            foreign_keys.push(foreign_titles);
            foreign_rows.iter().for_each(|r| {
                foreign_keys.push(r.clone());
            });
        }

        Ok(vec![primary_keys, foreign_keys])
    }

}