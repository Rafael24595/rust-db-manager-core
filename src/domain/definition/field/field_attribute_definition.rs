use serde::Deserialize;

use super::field_attribute_default_definition::FieldAttributeDefaultDefinition;

#[derive(Clone, Deserialize)]
pub struct FieldAttributeDefinition {
    name: String,
    code: String,
    values: Vec<FieldAttributeDefaultDefinition>,
}

impl FieldAttributeDefinition {
    
    pub fn new(name: String, code: String, values: Vec<FieldAttributeDefaultDefinition>) -> Self {
        Self {
            name, code, values
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn values(&self) -> Vec<FieldAttributeDefaultDefinition> {
        self.values.clone()
    }

}