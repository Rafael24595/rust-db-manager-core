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
    
    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn values(&self) -> Vec<FilterAttributeDefaultDefinition> {
        self.values.clone()
    }

    pub fn applies(&self) -> Vec<EFilterCategory> {
        self.applies.clone()
    }

}