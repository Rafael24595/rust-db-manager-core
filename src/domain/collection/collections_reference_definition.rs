use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionReferenceDefinition {
    collection: String,
    fields: Vec<String>,
}

impl CollectionReferenceDefinition {
    
    pub fn new(collection: String, fields: Vec<String>) -> Self {
        Self {
            collection, fields
        }
    }

    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    pub fn fields(&self) -> &Vec<String> {
        &self.fields
    }

}