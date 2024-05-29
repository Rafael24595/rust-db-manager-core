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

    pub fn data_base(&self) -> String {
        return self.data_base.clone();
    }

    pub fn collection(&self) -> String {
        return self.collection.clone();
    }

}