use super::filter_element::FilterElement;

//TODO: Add sort parameter.
#[derive(Clone)]
pub struct DocumentQuery {
    data_base: String,
    collection: String,
    skip: Option<usize>,
    limit: Option<usize>,
    filter: Option<FilterElement>
}

impl DocumentQuery {

    pub fn from_filter(data_base: String, collection: String, filter: FilterElement) -> Self {
        Self::from(data_base, collection, None, None, Some(filter))
    }
    
    pub fn from_query_unpaginated(query: &DocumentQuery) -> Self {
        Self::from(query.data_base().to_string(), query.collection().to_string(), None, None, query.filter().clone())
    }

    pub fn from(data_base: String, collection: String, skip: Option<usize>, limit: Option<usize>, filter: Option<FilterElement>) -> Self {
        Self {
            data_base: data_base,
            collection: collection,
            limit: limit,
            skip: skip,
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

    pub fn filter(&self) -> &Option<FilterElement> {
        &self.filter
    }

}