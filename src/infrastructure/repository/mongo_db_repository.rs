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
        connection_data::ConnectionData, data_base_group_data::DataBaseDataGroup, definition::field::{e_field_category::EFieldCategory, e_field_code::EFieldCode, field_attribute_default_definition::FieldAttributeDefaultDefinition, field_attribute_definition::FieldAttributeDefinition, field_definition::FieldDefinition}, filter::{data_base_query::DataBaseQuery, filter_element::FilterElement}, generate::{generate_collection_query::GenerateCollectionQuery, generate_database_query::GenerateDatabaseQuery}
    }
};

use super::{extractor_metadata_mongo_db::ExtractorMetadataMongoDb, i_db_repository::IDBRepository};

#[derive(Clone)]
pub struct MongoDbRepository {
    client: Client
}

impl MongoDbRepository {
    
    pub async fn new(connection: &ConnectionData) -> Result<impl IDBRepository, ConnectException> {
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
        
        self.collection(&data_base, &collection)
    }

    fn collection_from_resource(&self, query: &GenerateCollectionQuery) -> Collection<Document> {
        let data_base = query.data_base();        
        let collection = query.collection();

        self.collection(&data_base, &collection)
    }

    fn data_base(&self, data_base: &String) -> Database {
        self.client.database(&data_base)
    }

    fn collection(&self, data_base: &String, collection: &String) -> Collection<Document> {
        self.data_base(data_base).collection(&collection)
    }

    async fn find_cursor(&self, query: &DataBaseQuery) -> Result<Cursor<Document>, ConnectException>  {
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

        Ok(r_cursor.unwrap())
    }

    async fn collections_metadata_document(&self, data_base: String, collection: String) -> Result<Document, ConnectException> {
        let data_base = self.data_base(&data_base);
        Ok(data_base
            .run_command(doc! {"collStats": collection}, None).await.unwrap())
    }

}


#[async_trait]
impl IDBRepository for MongoDbRepository {

    async fn status(&self) -> Result<(), ConnectException> {
        let _ = self.list_data_bases().await?;
        return Ok(());
    }

    async fn data_base_metadata(&self) -> Result<Vec<DataBaseDataGroup>, ConnectException> {
        let server_info = &self.client.database("admin")
            .run_command(doc! {"serverStatus": 1}, None).await.unwrap();

        ExtractorMetadataMongoDb::from_db(server_info)
    }

    async fn data_base_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        let databases = self.list_data_bases().await?;

        Ok(databases.iter().any(|name| name == &query.data_base()))
    }

    async fn create_data_base(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let temp_col = format!("TEMP_{}", Uuid::new_v4().to_string());
        if self.collection_exists(&DataBaseQuery::from(data_base.clone(), temp_col.clone())).await? {
            return self.create_data_base(query).await;
        }

        let query = GenerateCollectionQuery::new(data_base.clone(), temp_col);
        let _ = self.create_collection(&query).await?;

        Ok(data_base)
    }

    async fn drop_data_base(&self, query: &GenerateDatabaseQuery) -> Result<String, ConnectException> {
        let data_base = query.data_base();
        let database = self.data_base(&data_base);
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

    async fn data_base_collections_metadata(&self, query: &DataBaseQuery) -> Result<Vec<DataBaseDataGroup>, ConnectException> {
        let mut documents = Vec::new();
        
        let collections = self.list_collections(query).await?;
        for collection in collections {
            let document = self.collections_metadata_document(query.data_base(), collection).await?;
            documents.push(document);
        }

        ExtractorMetadataMongoDb::from_collections(documents)
    }

    async fn collection_accept_definition(&self) -> Result<Vec<FieldDefinition>, ConnectException> {
        let mut definition = Vec::new();

        let index_attributes = Vec::from(vec![
            FieldAttributeDefinition::new(
                String::from("Direction"),
                String::from("Direction"),
                Vec::from(vec![
                    FieldAttributeDefaultDefinition::new(String::from("ASC"), String::from("1")),
                    FieldAttributeDefaultDefinition::new(String::from("DSC"), String::from("-1")),
                ])
            )
        ]);

        definition.push(
            FieldDefinition::new(0, String::from("Index"), EFieldCode::ID, EFieldCategory::STRING, false, true, index_attributes)
        );

        Ok(definition)
    }

    async fn collection_metadata(&self, query: &DataBaseQuery) -> Result<Vec<DataBaseDataGroup>, ConnectException> {
        let document = self.collections_metadata_document(query.data_base(), query.collection()).await?;

        ExtractorMetadataMongoDb::from_collection(document)
    }

    async fn collection_exists(&self, query: &DataBaseQuery) -> Result<bool, ConnectException> {
        let collections = self.find(query).await?;
        
        Ok(collections.iter().any(|name| name == &query.collection()))
    }

    async fn create_collection(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        let collection = query.collection();
        let db = self.data_base(&query.data_base());
        let result = db.create_collection(query.collection(), None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }

        Ok(collection)
    }

    async fn drop_collection(&self, query: &GenerateCollectionQuery) -> Result<String, ConnectException> {
        let collection = self.collection_from_resource(&query);
        let result = collection.drop(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.err().unwrap().to_string());
            return Err(exception);
        }

        Ok(query.collection())
    }

    async fn list_collections(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let result = self.data_base(&query.data_base()).list_collection_names(None).await;
        if result.is_err() {
            let exception = ConnectException::new(result.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(result.ok().unwrap())
    }

    async fn find(&self, query: &DataBaseQuery) -> Result<Option<String>, ConnectException> {
        let o_result = self.find_query(query).await;
        if o_result.is_err() {
            return Err(o_result.unwrap_err());
        }
        
        Ok(o_result.unwrap().first().cloned())
    }

    async fn find_query_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
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

    async fn find_query(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
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

    async fn find_all_lite(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let fix = DataBaseQuery::from(query.data_base(), query.collection());
        return self.find_query_lite(&fix).await;
    }

    async fn find_all(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
        let fix = DataBaseQuery::from(query.data_base(), query.collection());
        return self.find_query(&fix).await;
    }

    async fn insert(&self, query: &DataBaseQuery, value: String) -> Result<String, ConnectException> {
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

    async fn update(&self, query: &DataBaseQuery, value: String) -> Vec<u8> {
        todo!()
    }

    async fn delete(&self, query: &DataBaseQuery) -> Result<Vec<String>, ConnectException> {
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