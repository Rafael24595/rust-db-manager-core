use async_trait::async_trait;
use mongodb::{
    bson::{doc, to_document, Document}, options::{AggregateOptions, ClientOptions}, Client, Collection, Cursor, Database
};

use futures_util::stream::StreamExt;
use serde_json::{
    from_str, 
    Value
};
use uuid::Uuid;

use crate::{
    commons::exception::connect_exception::ConnectException, 
    domain::{
        connection_data::ConnectionData, data_base_info::DataBaseInfo, filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}, generate::generate_resource_query::GenerateResourceQuery
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

        
    fn collection_from_query(&self, query: &DataBaseQuery) -> Collection<Document> {
        let data_base = query.data_base();        
        let collection = query.collection();
        return self.data_base(data_base).collection(&collection);
    }

    fn collection_from_resource(&self, query: &GenerateResourceQuery) -> Collection<Document> {
        let data_base = query.data_base();        
        let collection = query.collection();
        return self.data_base(data_base).collection(&collection);
    }

    fn data_base(&self, data_base: String) -> Database {
        return self.client.database(&data_base);
    }

    fn collection(&self, data_base: String, collection: String) -> Collection<Document> {
        return self.data_base(data_base).collection(&collection);
    }

    async fn find_cursor(&self, query: DataBaseQuery) -> Result<Cursor<Document>, ConnectException>  {
        let collection = self.collection_from_query(&query);

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

        return Ok(r_cursor.unwrap());
    }

}

#[async_trait]
impl IDBRepository for MongoDbRepository {

    async fn status(&self) -> Result<(), ConnectException> {
        let _ = self.list_data_bases().await?;
        return Ok(());
    }

    async fn info(&self) -> DataBaseInfo {
        let server_info = self.client.database("admin")
            .run_command(doc! {"serverStatus": 1}, None).await.unwrap();
        todo!()
    }

    async fn data_base_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException> {
        let databases = self.list_data_bases().await?;

        Ok(databases.iter().any(|name| name == &query.data_base()))
    }

    async fn create_data_base(&self, query: GenerateResourceQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let temp_col = format!("TEMP_{}", Uuid::new_v4().to_string());
        if self.collection_exists(DataBaseQuery::from(data_base.clone(), temp_col.clone())).await? {
            return self.create_data_base(query).await;
        }

        let query = GenerateResourceQuery::new(data_base.clone(), temp_col);
        let _ = self.create_collection(query).await?;

        Ok(data_base)
    }

    async fn drop_data_base(&self, query: GenerateResourceQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let database = self.data_base(data_base.clone());
        let result = database.drop(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }

        Ok(data_base)
    }

    async fn list_data_bases(&self) -> Result<Vec<String>, ConnectException> {
        let result = self.client.list_database_names(None, None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }
        
        Ok(result.ok().unwrap())
    }

    async fn collection_exists(&self, query: DataBaseQuery) -> Result<bool, ConnectException> {
        let collections = self.list_collections(query.clone()).await?;
        
        Ok(collections.iter().any(|name| name == &query.collection()))
    }

    async fn create_collection(&self, query: GenerateResourceQuery) -> Result<String, ConnectException> {
        let collection = query.collection();
        let db = self.data_base(query.data_base());
        let result = db.create_collection(query.collection(), None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }

        Ok(collection)
    }

    async fn drop_collection(&self, query: GenerateResourceQuery) -> Result<String, ConnectException> {
        let collection = self.collection_from_resource(&query);
        let result = collection.drop(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }

        Ok(query.collection())
    }

    async fn list_collections(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let result = self.data_base(query.data_base()).list_collection_names(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(result.ok().unwrap())
    }

    async fn find(&self, query: DataBaseQuery) -> Result<Option<String>, ConnectException> {
        let o_result = self.find_query(query).await;
        if o_result.is_err() {
            return Err(o_result.unwrap_err());
        }
        
        Ok(o_result.unwrap().first().cloned())
    }

    async fn find_query_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let mut elements = Vec::<String>::new();
        
        let mut cursor = self.find_cursor(query).await?;
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

    async fn find_query(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let mut elements = Vec::<String>::new();
        
        let mut cursor = self.find_cursor(query).await?;
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

    async fn find_all_lite(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.find_query_lite(query).await;
    }

    async fn find_all(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        return self.find_query(query).await;
    }

    async fn insert(&self, query: DataBaseQuery, value: String) -> Result<String, ConnectException> {
        let collection = self.collection_from_query(&query);

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

    async fn update(&self, query: DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    async fn delete(&self, query: DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let collection = self.collection_from_query(&query);

        let mut elements_deleted = Vec::<String>::new();

        let mut cursor = self.find_cursor(query).await?;
        while let Some(r_document) = cursor.next().await {
            if r_document.is_err() {
                let exception = ConnectException::new(r_document.unwrap_err().to_string());
                return Err(exception);
            }
    
            let document = r_document.unwrap();
    
            let result = collection.delete_one(document.clone(), None).await;
            if result.is_err() {
                let exception = ConnectException::new(result.unwrap_err().to_string());
                return Err(exception);
            }

            let id = document.get("_id");
            if id.is_some() {
                elements_deleted.push(format!("_id={}", id.unwrap().to_string()));
            }
        }
     
        Ok(elements_deleted)
    }
    
}