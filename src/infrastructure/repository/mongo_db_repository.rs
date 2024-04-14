use async_trait::async_trait;
use mongodb::{bson::Document, options::{AggregateOptions, ClientOptions}, Client, Collection, Database};

use crate::{
    commons::exception::connect_exception::ConnectException, 
    domain::{
        connection_data::ConnectionData, 
        filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}
    }
};

use super::i_db_repository::IDBRepository;

#[derive(Clone)]
pub struct MongoDbRepository {
    client: Client
}

impl MongoDbRepository {
    
    pub async fn new(connection: ConnectionData) -> Result<impl IDBRepository, ConnectException> {
        let client = MongoDbRepository::connect(connection.connection()).await;
        if client.is_err() {
            let exception = ConnectException::new(client.err().unwrap().to_string());
            return Err(exception);
        }
        
        let instance = MongoDbRepository {
            client: client.ok().unwrap()
        };

        Ok(instance)
    }

    async fn connect(connection: String) -> Result<Client, mongodb::error::Error> {
        let client_options = ClientOptions::parse(connection).await?;
        let client = Client::with_options(client_options)?;
        Ok(client)
    }

    fn data_base(&self, query: &DataBaseQuery) -> Database {
        let data_base = query.data_base();
        return self.client.database(&data_base);
    }

    fn collection(&self, query: &DataBaseQuery) -> Collection<Document> {
        let collection = query.collection();
        return self.data_base(query).collection(&collection);
    }

}

#[async_trait]
impl IDBRepository for MongoDbRepository {

    async fn status(&self) -> Result<(), ConnectException> {
        let result = self.client.list_database_names(None, None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }
        return Ok(());
    }

    fn info(&self) -> Vec<u8> {
        todo!()
    }

    async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException> {
        let result = self.client.list_database_names(None, None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }
        return Ok(result.ok().unwrap());
    }

    async fn list_collections(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let result = self.data_base(&query).list_collection_names(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }
        return Ok(result.ok().unwrap());
    }

    async fn find(&self, query: DataBaseQuery) -> Result<Vec<u8>, ConnectException> {
        let collection = self.collection(&query);
        
        let mut filter = FilterElement::new();

        let o_filter = query.filter();
        if o_filter.is_some() {
            filter = o_filter.unwrap();
        }

        let pipeline: Result<Vec<Document>, ConnectException> = filter.as_mongo_agregate();
        if pipeline.is_err() {
            return Err(pipeline.err().unwrap());
        }
    
        let cursor = collection.aggregate(pipeline.ok().unwrap(), AggregateOptions::default()).await;

        todo!()
    }

    fn find_all(&self, query: DataBaseQuery) -> Vec<String> {
        todo!()
    }

    fn insert(&self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    fn update(&self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    fn delete(&self, query: DataBaseQuery) -> Vec<u8> {
        todo!()
    }

}