use super::filter_element::FilterElement;

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
    
    pub fn from(data_base: String, collection: String, skip: Option<usize>, limit: Option<usize>, filter: Option<FilterElement>) -> Self {
        Self {
            data_base: data_base,
            collection: collection,
            limit: limit,
            skip: skip,
            filter: filter
        }
    }

    pub fn data_base(&self) -> String {
        return self.data_base.clone();
    }

    pub fn collection(&self) -> String {
        return self.collection.clone();
    }

    pub fn skip(&self) -> Option<usize> {
        return self.skip;
    }

    pub fn limit(&self) -> Option<usize> {
        return self.limit;
    }

    pub fn filter(&self) -> Option<FilterElement> {
        return self.filter.clone();
    }

}