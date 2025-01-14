#[derive(Debug, Clone)]
pub struct DocumentData {
    data_base: String,
    collection: String,
    document: String
}

impl DocumentData {
    
    pub fn new(data_base: String, collection: String, document: String) -> Self {
        Self {
            data_base, collection, document
        }
    }

    pub fn data_base(&self) -> String {
        self.data_base.clone()
    }

    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    pub fn document(&self) -> String {
        self.document.clone()
    }

}