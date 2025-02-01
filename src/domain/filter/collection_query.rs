#[derive(Clone)]
pub struct CollectionQuery {
    data_base: String,
    collection: String
}

impl CollectionQuery {
    
    pub fn from(data_base: String, collection: String) -> Self {
        Self {
            data_base, collection
        }
    }

    pub fn data_base(&self) -> &str {
        &self.data_base
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

}