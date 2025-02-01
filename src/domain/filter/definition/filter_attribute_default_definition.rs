use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FilterAttributeDefaultDefinition {
    key: String,
    value: String,
    default: bool,
}

impl FilterAttributeDefaultDefinition {
    
    pub fn new(key: String, value: String, default: bool) -> Self {
        Self {
            key, value, default
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn default(&self) -> bool {
        self.default
    }

}