use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FieldReference {
    collection: String,
    field: String,
    cascade: bool
}

impl FieldReference {
    
    pub fn new(collection: String, field: String, cascade: bool) -> Self {
        Self {
            collection, field, cascade
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn cascade(&self) -> bool {
        self.cascade
    }

}