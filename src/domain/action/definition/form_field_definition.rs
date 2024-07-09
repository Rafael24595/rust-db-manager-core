use serde::Deserialize;

use super::form_default::FormDefault;

#[derive(Clone, Deserialize)]
pub struct FormFieldDefinition {
    order: usize,
    code: String,
    name: String,
    sw_key: bool,
    values: Vec<FormDefault>,
}

impl FormFieldDefinition {
    
    pub fn new(order: usize, code: String, name: String, sw_key: bool, values: Vec<FormDefault>) -> Self {
        Self {
            order, code, name, sw_key, values
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }
    
    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_key(&self) -> bool {
        self.sw_key
    }

    pub fn values(&self) -> Vec<FormDefault> {
        self.values.clone()
    }

}