#[derive(Clone)]
pub struct FormField {
    code: String,
    value: String,
}

impl FormField {
    
    pub fn new(code: String, value: String) -> Self {
        Self {
            code, value
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn value(&self) -> &str {
        &self.value
    }

}