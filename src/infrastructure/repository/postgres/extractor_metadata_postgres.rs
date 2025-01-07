use chrono::{DateTime, Utc};
use tokio_postgres::Client;

use crate::{commons::exception::connect_exception::ConnectException, domain::table::group::{e_data_type::EDataType, table_data_group::TableDataGroup}};

pub(crate) struct ExtractorMetadataPostgres {
}

impl ExtractorMetadataPostgres {

    pub(crate) async fn from_db(client: &Client) -> Result<Vec<TableDataGroup>, ConnectException> {
        let mut metadata: Vec<TableDataGroup> = Vec::new();
        
        metadata.push(Self::metadata_general(client).await?);

        Ok(metadata)
    }
    
    async fn metadata_general(client: &Client) -> Result<TableDataGroup, ConnectException> {
        let mut group = TableDataGroup::new(0, String::from("general"));

        group.push(
            String::from("Version"),
            Self::version(client).await?
        );

        group.push_typed(
            String::from("Uptime"),
           Self::timeup(client).await?,
           EDataType::TIMESTAMP
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

}