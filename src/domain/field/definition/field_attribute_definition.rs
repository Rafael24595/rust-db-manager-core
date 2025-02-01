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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn values(&self) -> &Vec<FieldAttributeDefaultDefinition> {
        &self.values
    }

}