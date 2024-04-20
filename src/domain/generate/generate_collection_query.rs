#[derive(Clone)]
pub struct GenerateCollectionQuery {
    data_base: String,
    collection: String
}

impl GenerateCollectionQuery {

    pub fn from_data_base(data_base: String) -> GenerateCollectionQuery {
        GenerateCollectionQuery {
            data_base: data_base,
            collection: String::new()
        }
    }

    pub fn new(data_base: String, collection: String) -> GenerateCollectionQuery {
        GenerateCollectionQuery {
            data_base: data_base,
            collection: collection
        }
    }

    pub fn data_base(&self) -> String {
        return self.data_base.clone();
    }

    pub fn collection(&self) -> String {
        return self.collection.clone();
    }

}