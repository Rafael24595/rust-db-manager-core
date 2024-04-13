use async_trait::async_trait;
use mongodb::{bson::Document, options::ClientOptions, Client, Collection};

use crate::{
    commons::exception::connect_exception::ConnectException, 
    domain::{
        connection_data::ConnectionData, 
        filter::data_base_query::DataBaseQuery
    }
};

use super::i_db_repository::IDBRepository;

pub struct MongoDbRepository {
    client: Client
}

impl MongoDbRepository {
    
    pub async fn new(connection: ConnectionData) -> Result<impl IDBRepository, ConnectException> {
        let client = MongoDbRepository::connect(String::new()).await;
        if client.is_err() {
            let exception = ConnectException::new(connection.connection());
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

    fn collection(self, query: DataBaseQuery) -> Collection<Document> {
        let data_base = query.data_base();
        let collection = query.collection();
        return self.client.database(&data_base).collection(&collection);
    }

}

#[async_trait]
impl IDBRepository for MongoDbRepository {

    async fn status(self) -> Result<(), ConnectException> {
        let result = self.client.list_database_names(None, None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }
        return Ok(());
    }

    fn info(self) -> Vec<u8> {
        todo!()
    }

    fn find(self, query: DataBaseQuery) -> Vec<u8> {
        todo!()
    }

    fn find_all(self, query: DataBaseQuery) -> Vec<String> {
        todo!()
    }

    fn insert(self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    fn update(self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    fn delete(self, query: DataBaseQuery) -> Vec<u8> {
        todo!()
    }

}