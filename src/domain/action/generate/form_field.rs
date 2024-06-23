#[derive(Clone)]
pub struct FormField {
    code: String,
    value: Vec<String>,
}

impl FormField {
    
    pub fn new(code: String, value: Vec<String>) -> Self {
        Self {
            code, value
        }
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn value(&self) -> Vec<String> {
        self.value.clone()
    }

}