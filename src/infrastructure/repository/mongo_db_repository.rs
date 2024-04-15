use async_trait::async_trait;
use mongodb::{
    bson::{to_document, Document},
    options::{AggregateOptions, ClientOptions},
    Client, Collection, Database
};

use futures_util::stream::StreamExt;
use serde_json::{
    from_str, 
    Value
};

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
            let exception = ConnectException::new(result.unwrap_err().to_string());
            return Err(exception);
        }
        return Ok(result.ok().unwrap());
    }

    async fn find(&self, query: DataBaseQuery) -> Result<Option<String>, ConnectException> {
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
    
        let r_cursor = collection.aggregate(pipeline.ok().unwrap(), AggregateOptions::default()).await;
        if r_cursor.is_err() {
            let exception = ConnectException::new(r_cursor.unwrap_err().to_string());
            return Err(exception);
        }

        let mut cursor = r_cursor.ok().unwrap();

        let r_document = cursor.next().await;
        if r_document.is_none() {
            return Ok(None);
        }

        let document = r_document.unwrap();

        if document.is_err() {
            let exception = ConnectException::new(document.unwrap_err().to_string());
            return Err(exception);
        }

        let json = serde_json::to_string(&document.unwrap());
        if json.is_err() {
            let exception = ConnectException::new(json.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(Some(json.unwrap()))
    }

    async fn find_all_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let collection = self.collection(&query);

        let r_cursor = collection.find(None, None).await;
        if r_cursor.is_err() {
            let exception = ConnectException::new(r_cursor.unwrap_err().to_string());
            return Err(exception);
        }

        let mut elements = Vec::<String>::new();
        
        let mut cursor = r_cursor.ok().unwrap();
        while let Some(result) = cursor.next().await {
            if result.is_err() {
                let exception = ConnectException::new(result.unwrap_err().to_string());
                return Err(exception);
            }

            let document = result.ok().unwrap();

            let id = document.get("_id");
            if id.is_some() {
                elements.push(format!("_id={}", id.unwrap().to_string()));
            }
        }

        Ok(elements)
    }

    async fn find_all(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let collection = self.collection(&query);

        let r_cursor = collection.find(None, None).await;
        if r_cursor.is_err() {
            let exception = ConnectException::new(r_cursor.unwrap_err().to_string());
            return Err(exception);
        }

        let mut elements = Vec::<String>::new();
        
        let mut cursor = r_cursor.ok().unwrap();
        while let Some(result) = cursor.next().await {
            if result.is_err() {
                let exception = ConnectException::new(result.unwrap_err().to_string());
                return Err(exception);
            }

            let document = result.ok().unwrap();

            let json = serde_json::to_string(&document);
            if json.is_err() {
                let exception = ConnectException::new(json.unwrap_err().to_string());
                return Err(exception);
            }
            elements.push(json.ok().unwrap());
        }

        Ok(elements)
    }

    async fn insert(&self, query: DataBaseQuery, value: String) -> Result<String,ConnectException> {
        let collection = self.collection(&query);

        let json: Result<Value, _> = from_str(&value);

        if json.is_err() {
            let error_message = format!("Invalid JSON format: {}", json.err().unwrap());
            let exception = ConnectException::new(error_message);
            return Err(exception);
        }

        let document = to_document(&json.unwrap());

        if document.is_err(){
            let err = format!("Failed to convert JSON to BSON: {}", document.unwrap_err());
            return Err(ConnectException::new(err));
        }


        let result = collection.insert_one(document.unwrap(), None).await;
        if result.is_err() {
            let err = format!("Could not insert into database: {}", result.err().unwrap());
            return Err(ConnectException::new(err));
        }

        let id =result.unwrap().inserted_id.as_object_id();
        if id.is_none(){
            let err = format!("Failed to get ID");
            return Err(ConnectException::new(err));
        }

        Ok(id.unwrap().to_string())
    }

    fn update(&self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    fn delete(&self, query: DataBaseQuery) -> Vec<u8> {
        todo!()
    }

}