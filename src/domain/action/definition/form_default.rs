use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FormDefault {
    key: String,
    value: String
}

impl FormDefault {
    
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