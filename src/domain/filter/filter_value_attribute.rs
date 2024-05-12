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

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

}