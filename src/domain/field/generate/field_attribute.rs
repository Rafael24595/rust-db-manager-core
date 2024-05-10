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

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

}