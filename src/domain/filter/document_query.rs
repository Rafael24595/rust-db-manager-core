use crate::domain::document::document_key::DocumentKey;

use super::filter_element::FilterElement;

//TODO: Add sort parameter.
#[derive(Clone)]
pub struct DocumentQuery {
    data_base: String,
    collection: String,
    skip: Option<usize>,
    limit: Option<usize>,
    keys: Vec<DocumentKey>,
    filter: Option<FilterElement>
}

impl DocumentQuery {

    pub fn from_keys(data_base: String, collection: String, keys: Vec<DocumentKey>) -> Self {
        Self::from(data_base, collection, None, None, keys, None)
    }

    pub fn from_filter(data_base: String, collection: String, filter: FilterElement) -> Self {
        Self::from(data_base, collection, None, None, Vec::new(), Some(filter))
    }
    
    pub fn from_query_unpaginated(query: &DocumentQuery) -> Self {
        Self::from(query.data_base().to_string(), query.collection().to_string(), None, None, query.keys().clone(), query.filter().clone())
    }

    pub fn from(data_base: String, collection: String, skip: Option<usize>, limit: Option<usize>, keys: Vec<DocumentKey>, filter: Option<FilterElement>) -> Self {
        Self {
            data_base: data_base,
            collection: collection,
            limit: limit,
            skip: skip,
            keys: keys,
            filter: filter
        }
    }

    pub fn data_base(&self) -> &str {
        &self.data_base
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn skip(&self) -> Option<usize> {
        self.skip
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn keys(&self) -> &Vec<DocumentKey> {
        &self.keys
    }

    pub fn filter(&self) -> &Option<FilterElement> {
        &self.filter
    }

}