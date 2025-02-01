use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FieldAttribute {
    key: String,
    value: String,
}

impl FieldAttribute {

    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

}