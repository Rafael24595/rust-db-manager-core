use serde::Deserialize;

use super::filter_attribute_default_definition::FilterAttributeDefaultDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterAttributeDefinition {
    code: String,
    name: String,
    description: String,
    values: Vec<FilterAttributeDefaultDefinition>,
    applies: Vec<String>,
}

impl FilterAttributeDefinition {
    
    pub fn new(code: String, name: String, description: String, values: Vec<FilterAttributeDefaultDefinition>, applies: Vec<String>) -> Self {
        Self {
            code, name, description, values, applies
        }
    }
    
    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn values(&self) -> &Vec<FilterAttributeDefaultDefinition> {
        &self.values
    }

    pub fn applies(&self) -> &Vec<String> {
        &self.applies
    }

}