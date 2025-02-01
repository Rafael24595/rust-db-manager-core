use serde::Deserialize;

use crate::domain::filter::e_filter_category::EFilterCategory;

use super::filter_attribute_default_definition::FilterAttributeDefaultDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterAttributeDefinition {
    code: String,
    name: String,
    description: String,
    values: Vec<FilterAttributeDefaultDefinition>,
    applies: Vec<EFilterCategory>,
}

impl FilterAttributeDefinition {
    
    pub fn new(code: String, name: String, description: String, values: Vec<FilterAttributeDefaultDefinition>, applies: Vec<EFilterCategory>) -> Self {
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

    pub fn applies(&self) -> &Vec<EFilterCategory> {
        &self.applies
    }

}