use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FieldReference {
    collection: String,
    field: String
}

impl FieldReference {
    
    pub fn new(collection: String, field: String) -> Self {
        Self {
            collection, field
        }
    }

    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    pub fn field(&self) -> String {
        self.field.clone()
    }

}