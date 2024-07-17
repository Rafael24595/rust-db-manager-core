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

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn default(&self) -> bool {
        self.default
    }

}