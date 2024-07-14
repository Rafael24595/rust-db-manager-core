use serde::Deserialize;

use super::filter_attribute_definition::FilterAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterDefinition {
    query_type: String,
    query_example: String,
    attributes: Vec<FilterAttributeDefinition>
}

impl FilterDefinition {

    pub fn query_type(&self) -> String {
        self.query_type.clone()
    }

    pub fn query_example(&self) -> String {
        self.query_example.clone()
    }

    pub fn attributes(&self) -> Vec<FilterAttributeDefinition> {
        self.attributes.clone()
    }

}