use super::document_key::DocumentKey;

#[derive(Debug, Clone)]
pub struct DocumentData {
    data_base: String,
    collection: String,
    base_key: Option<DocumentKey>,
    keys: Vec<DocumentKey>,
    document: String
}

impl DocumentData {
    
    pub fn new(data_base: String, collection: String, base_key: Option<DocumentKey>, keys: Vec<DocumentKey>, document: String) -> Self {
        Self {
            data_base, collection, base_key,
            keys, document
        }
    }

    pub fn data_base(&self) -> String {
        self.data_base.clone()
    }

    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    pub fn base_key(&self) -> Option<DocumentKey> {
        self.base_key.clone()
    }

    pub fn keys(&self) -> Vec<DocumentKey> {
        self.keys.clone()
    }

    pub fn document(&self) -> String {
        self.document.clone()
    }

}