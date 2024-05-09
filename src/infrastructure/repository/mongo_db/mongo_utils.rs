use mongodb::{bson::{doc, Document}, options::IndexOptions, IndexModel};
use serde_json::from_str;

use crate::{commons::exception::connect_exception::ConnectException, domain::{definition::field::e_field_code::EFieldCode, filter::{e_filter_category::EFilterCategory, filter_element::FilterElement, filter_value::FilterValue}, generate::field::field_data::FieldData}};

pub struct QueryItems {
    and_fields: Vec<String>,
    or_fields: Vec<String>,
    queries: Vec<String>
}

impl FilterElement {
    
    pub fn as_mongo_agregate(&self) -> Result<Vec<Document>, ConnectException> {
        let mut registry = QueryItems {and_fields: Vec::new(), or_fields: Vec::new(), queries: Vec::new()};
        registry = self._as_mongo_agregate(registry);

        let mut result = Vec::<String>::new();
        let mut matches_collection = Vec::<String>::new();

        if !registry.and_fields.is_empty() {
            let match_string = format!("\"$and\": [ {} ]", registry.and_fields.join(", "));
            matches_collection.push(match_string);
        }

        if !registry.or_fields.is_empty() {
            let match_string = format!("\"$or\": [ {} ]", registry.or_fields.join(", "));
            matches_collection.push(match_string);
        }

        if !matches_collection.is_empty() {
            let match_string = format!("{{ \"$match\": {{ {} }} }}", matches_collection.join(", "));
            result.push(match_string);
        }

        if !registry.queries.is_empty() {
            let query_string = registry.queries.join(", ");
            result.push(query_string);
        }

        let pipeline_str = &format!("[ {} ]", result.join(", "));

        let pipeline: Result<Vec<Document>, serde_json::Error> = from_str(pipeline_str);
        if pipeline.is_err() {
            let exception = ConnectException::new(pipeline.err().unwrap().to_string());
            return Err(exception);
        }

        return Ok(pipeline.ok().unwrap());
    }

    fn _as_mongo_agregate(&self, mut registry: QueryItems) -> QueryItems {
        let f_value = self.value();
        let field = self.field();

        let result = f_value.as_mongo_agregate(registry);
        let mut value = result.0;
        registry = result.1;

        let category = f_value.category();

        if category == EFilterCategory::ROOT {
            return registry;    
        }

        if category == EFilterCategory::COLLECTION {
            let mut block = Vec::<String>::new();

            if !registry.and_fields.is_empty() {
                let block_and = format!("\"$and\": [ {} ]", registry.and_fields.join(", "));
                block.push(block_and);
                registry.and_fields.clear();
            }

            if !registry.and_fields.is_empty() {
                let block_or = format!("\"$or\": [ {} ]", registry.or_fields.join(", "));
                block.push(block_or);
                registry.or_fields.clear();
            }

            if !block.is_empty() {
                let query = format!(" {{ {} }} ", block.join(", "));
                if self.is_or() {
                    registry.or_fields.push(query);
                } else {
                    registry.and_fields.push(query);
                }   
            }

            return registry;    
        }

        if category == EFilterCategory::QUERY {
            registry.queries.push(value);
            return registry;    
        }

        if self.is_negate() {
            value = format!("{{ \"$not\": {{ \"$eq\": {} }} }}", value);
        }

        let query = format!("{{ \"{}\": {} }}", field, value);

        if self.is_or() {
            registry.or_fields.push(query);
        } else {
            registry.and_fields.push(query);
        }

        return registry;
    }

}

impl FilterValue {
    
    pub fn as_mongo_agregate(&self, registry: QueryItems) -> (String, QueryItems) {
        let value = self.value();
        match self.category() {
            EFilterCategory::ID => (format!("\"{}\"", value), registry),
            EFilterCategory::QUERY => (value, registry),
            EFilterCategory::STRING => (format!("\"{}\"", value), registry),
            EFilterCategory::BOOLEAN => (value, registry),
            EFilterCategory::NUMERIC => (value, registry),
            EFilterCategory::COLLECTION => (value, self.collection_as_mongo_agregate(registry)),
            EFilterCategory::ROOT => (value, self.collection_as_mongo_agregate(registry)),
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