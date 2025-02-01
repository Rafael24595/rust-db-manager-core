use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CollectionReferenceDefinition {
    collection: String,
    fields: Vec<String>,
    cascade: bool
}

impl CollectionReferenceDefinition {
    
    pub fn new(collection: String, fields: Vec<String>, cascade: bool) -> Self {
        Self {
            collection, fields, cascade
        }
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn fields(&self) -> &Vec<String> {
        &self.fields
    }

    pub fn push(&mut self, field: &str) -> &Self {
        self.fields.push(field.to_string());
        self
    }

    pub fn cascade(&self) -> bool {
        self.cascade
    }

}