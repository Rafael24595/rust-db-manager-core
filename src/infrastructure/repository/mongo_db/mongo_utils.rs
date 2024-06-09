use mongodb::{bson::{doc, oid::ObjectId, Bson, Document}, options::IndexOptions, IndexModel};
use serde_json::from_str;

use crate::{
    commons::exception::connect_exception::ConnectException,
    domain::{
        field::{e_field_code::EFieldCode, generate::field_data::FieldData},
        filter::{
            e_filter_category::EFilterCategory, filter_element::FilterElement,
            filter_value::FilterValue,
        },
    },
};

use super::e_filter_attributes::EFilterAtributtes;

pub struct QueryItems {
    and_fields: Vec<Document>,
    or_fields: Vec<Document>,
    queries: Vec<Document>
}

impl FilterElement {
    
    pub fn as_mongo_agregate(&self) -> Result<Vec<Document>, ConnectException> {
        let mut registry = QueryItems {and_fields: Vec::new(), or_fields: Vec::new(), queries: Vec::new()};
        registry = self.make_agregate(registry);

        let mut result = doc! {};
        let mut matches_collection = doc! {};

        let mut pipeline = Vec::new();

        if !registry.and_fields.is_empty() {
            matches_collection.insert("$and",  registry.and_fields);
        }

        if !registry.or_fields.is_empty() {
            matches_collection.insert("$or",  registry.or_fields);
        }

        if !matches_collection.is_empty() {
            result.insert("$match",  matches_collection);
            pipeline.push(result);
        }

        if !registry.queries.is_empty() {
            pipeline.append(&mut registry.queries);
        }

        Ok(pipeline)
    }

    fn make_agregate(&self, mut registry: QueryItems) -> QueryItems {
        let f_value = self.value();
        let field = self.field();

        let result = f_value.as_mongo_agregate(registry);
        let value = result.0;
        registry = result.1;

        match f_value.category() {
            EFilterCategory::ROOT => registry,
            EFilterCategory::COLLECTION => self.make_collection(registry),
            EFilterCategory::QUERY => self.make_query(registry, value),
            _ => self.make_base(registry, field, value)
        }
    }

    fn make_collection(&self, mut registry: QueryItems) -> QueryItems {
        let mut block = doc! {};

        if !registry.and_fields.is_empty() {
            block.insert("$and",  registry.and_fields.clone());
            registry.and_fields.clear();
        }

        if !registry.and_fields.is_empty() {
            block.insert("$or",  registry.or_fields.clone());
            registry.or_fields.clear();
        }

        if !block.is_empty() {
            if self.is_or() {
                registry.or_fields.push(block);
            } else {
                registry.and_fields.push(block);
            }   
        }

        return registry;
    }

    fn make_query(&self, mut registry: QueryItems, value: Bson) -> QueryItems {
        let document = value.as_document();
        //TODO: Error
        registry.queries.push(document.cloned().unwrap());
        return registry;    
    }

    fn make_base(&self, mut registry: QueryItems, field: String, value: Bson) -> QueryItems {
        let query;
        if self.is_negate() {
            query = doc! {
                field: {
                    "$not": {
                        "$eq": value
                    }
                }
            };
        } else {
            query = doc! {field: value};
        }

        if self.is_or() {
            registry.or_fields.push(query);
        } else {
            registry.and_fields.push(query);
        }

        return registry;
    }

}

impl FilterValue {
 
    pub fn as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        match self.category() {
            EFilterCategory::ID_NUMERIC | EFilterCategory::ID_STRING => self.id_as_mongo_agregate(registry),
            EFilterCategory::QUERY => self.query_as_mongo_agregate(registry),
            EFilterCategory::STRING => self.string_as_mongo_agregate(registry),
            EFilterCategory::BOOLEAN => self.boolean_as_mongo_agregate(registry),
            EFilterCategory::NUMERIC => self.integer_as_mongo_agregate(registry),
            EFilterCategory::COLLECTION => self.collection_as_mongo_agregate(registry),
            EFilterCategory::ROOT => self.collection_as_mongo_agregate(registry),
        }
    }

    pub fn id_as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        let attributes = self.attributes();
        let mut value = Bson::String(self.value());
        
        let o_oid = attributes.iter().find(|a| a.key() == EFilterAtributtes::OID.to_string());
        if let Some(s_oid) = o_oid {
            let oid = s_oid.value().parse::<bool>();
            if oid.is_ok() && oid.unwrap() {
                let oid = ObjectId::parse_str(self.value());
                if oid.is_ok() {
                    value = Bson::ObjectId(oid.unwrap());
                }
            }
        }
        
        (value, registry)
    }

    pub fn query_as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        let value = self.value();
        let pipeline: Result<Document, serde_json::Error> = from_str(&value);
        //TODO: Error
        (Bson::Document(pipeline.unwrap()), registry)
    }

    pub fn string_as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        let attributes = self.attributes();
        let mut value = Bson::String(self.value());

        let o_regex = attributes.iter().find(|a| a.key() == EFilterAtributtes::REGEX.to_string());
        if let Some(s_regex) = o_regex {
            let regex = s_regex.value().parse::<bool>();
            if regex.is_ok() && regex.unwrap() {
                value = Bson::Document(doc! {"$regex" : self.value()});
            }
        }

        (value, registry)
    }

    pub fn boolean_as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        let value = self.value();
        let boolean = value.parse::<bool>();
        //TODO: Error
        (Bson::Boolean(boolean.unwrap()), registry)
    }

    pub fn integer_as_mongo_agregate(&self, registry: QueryItems) -> (Bson, QueryItems) {
        let value = self.value();
        let integer = value.parse::<i64>();
        //TODO: Error
        (Bson::Int64(integer.unwrap()), registry)
    }

    fn collection_as_mongo_agregate(&self, mut registry: QueryItems) -> (Bson, QueryItems) {
        let value = self.value();
        for child in self.children() {
            registry = child.make_agregate(registry);
        }
        return (Bson::String(value), registry);
    }

}

impl FieldData {
    
    pub fn collection_as_mongo_create(collection: Vec<FieldData>) -> Result<Vec<IndexModel>, ConnectException>  {
        collection.iter()
            .map(|f| f.as_mongo_create())
            .collect()
    }

    pub fn as_mongo_create(&self) -> Result<IndexModel, ConnectException> {
        if self.code() != EFieldCode::INDEXED {
            let exception = ConnectException::new(String::from("Field type not supported."));
            return Err(exception);
        }

        let key = self.value();

        let attributes = self.attributes();

        let o_direction = attributes.iter()
            .find(|a| a.key() == "DIRECTION");

        let mut direction = 1;
        if let Some(value) = o_direction {
            direction = value.value().parse::<i32>().unwrap_or(1);
        }

        let o_unique = attributes.iter()
            .find(|a| a.key() == "UNIQUE");

        let mut unique = true;
        if let Some(value) = o_unique {
            unique = value.value().parse::<bool>().unwrap_or(true);
        }

        let mut opts = IndexOptions::builder().build();
        if key != "_id" {
            opts = IndexOptions::builder()
                .unique(unique)
                .build();
        }

        let index = IndexModel::builder()
            .keys(doc! { key: direction })
            .options(opts)
            .build();

        Ok(index)
    }

}