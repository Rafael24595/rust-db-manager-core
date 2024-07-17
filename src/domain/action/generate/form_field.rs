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

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

}