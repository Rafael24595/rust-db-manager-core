use super::filter_element::FilterElement;

pub struct DataBaseQuery {
    data_base: String,
    collection: String,
    filter: Option<FilterElement>
}

impl DataBaseQuery {
    
    pub fn from(data_base: String, collection: String) -> DataBaseQuery {
        DataBaseQuery {
            data_base: data_base,
            collection: collection,
            filter: None
        }
    }

    pub fn from_filter(data_base: String, collection: String, filter: FilterElement) -> DataBaseQuery {
        DataBaseQuery {
            data_base: data_base,
            collection: collection,
            filter: Some(filter)
        }
    }

    pub fn data_base(&self) -> String {
        return self.data_base.clone();
    }

    pub fn collection(&self) -> String {
        return self.collection.clone();
    }

}