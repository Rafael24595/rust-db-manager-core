use serde::Deserialize;

use super::filter_attribute_definition::FilterAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterDefinition {
    query_type: String,
    query_example: String,
    attributes: Vec<FilterAttributeDefinition>
}

impl FilterDefinition {

    pub fn query_type(&self) -> &str {
        &self.query_type
    }

    pub fn query_example(&self) -> &str {
        &self.query_example
    }

    pub fn attributes(&self) -> &Vec<FilterAttributeDefinition> {
        &self.attributes
    }

}