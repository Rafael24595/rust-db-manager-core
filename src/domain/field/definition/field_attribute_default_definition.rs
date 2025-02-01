use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FieldAttributeDefaultDefinition {
    key: String,
    value: String
}

impl FieldAttributeDefaultDefinition {
    
    pub fn new(key: String, value: String) -> Self {
        Self {
            key, value
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

}