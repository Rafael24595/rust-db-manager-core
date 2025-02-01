#[derive(Debug, Clone)]
pub struct FilterValueAttribute {
    key: String,
    value: String
}

impl FilterValueAttribute {
    
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