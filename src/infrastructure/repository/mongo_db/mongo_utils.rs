use mongodb::{bson::{doc, oid::ObjectId, Bson, Document}, options::IndexOptions, IndexModel};
use serde_json::from_str;

use crate::{commons::exception::connect_exception::ConnectException, domain::{field::{e_field_code::EFieldCode, generate::field_data::FieldData}, filter::{e_filter_category::EFilterCategory, filter_element::FilterElement, filter_value::FilterValue}}};

pub struct QueryItems {
    and_fields: Vec<Document>,
    or_fields: Vec<Document>,
    queries: Vec<Document>
}

impl FilterElement {
    
    pub fn as_mongo_agregate(&self) -> Result<Vec<Document>, ConnectException> {
        let mut registry = QueryItems {and_fields: Vec::new(), or_fields: Vec::new(), queries: Vec::new()};
        registry = self._as_mongo_agregate(registry);

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

    fn _as_mongo_agregate(&self, mut registry: QueryItems) -> QueryItems {
        let f_value = self.value();
        let field = self.field();

        let result = f_value.as_mongo_agregate(registry);
        let value = result.0;
        registry = result.1;

        let category = f_value.category();

        if category == EFilterCategory::ROOT {
            return registry;    
        }

        if category == EFilterCategory::COLLECTION {
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

        if category == EFilterCategory::QUERY {
            let document = value.as_document();
            //TODO: Error
            registry.queries.push(document.cloned().unwrap());
            return registry;    
        }

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
        let value = self.value();
        match self.category() {
            EFilterCategory::ID_NUMERIC | EFilterCategory::ID_STRING => {
                let attributes = self.attributes();
                let oid = attributes.iter().find(|a| a.key() == "$oid");
                if let Some(_) = oid {
                    let oid = ObjectId::parse_str(value);
                    //TODO: Error
                    return (Bson::ObjectId(oid.unwrap()), registry);
                }
                (Bson::String(value), registry)
            },
            EFilterCategory::QUERY => {
                let pipeline: Result<Document, serde_json::Error> = from_str(&value);
                //TODO: Error
                (Bson::Document(pipeline.unwrap()), registry)
            },
            EFilterCategory::STRING => (Bson::String(value), registry),
            EFilterCategory::BOOLEAN => {
                let boolean = value.parse::<bool>();
                //TODO: Error
                (Bson::Boolean(boolean.unwrap()), registry)
            },
            EFilterCategory::NUMERIC => {
                let integer = value.parse::<i64>();
                //TODO: Error
                (Bson::Int64(integer.unwrap()), registry)
            },
            EFilterCategory::COLLECTION => (Bson::String(value), self.collection_as_mongo_agregate(registry)),
            EFilterCategory::ROOT => (Bson::String(value), self.collection_as_mongo_agregate(registry)),
        }
    }

    fn collection_as_mongo_agregate(&self, mut registry: QueryItems) -> QueryItems {
        for child in self.children() {
            registry = child._as_mongo_agregate(registry);
        }
        return registry;
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