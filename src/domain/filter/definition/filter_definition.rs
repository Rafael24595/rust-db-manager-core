use serde::Deserialize;

use super::filter_attribute_definition::FilterAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterDefinition {
    attributes: Vec<FilterAttributeDefinition>
}

impl FilterDefinition {

    pub fn attributes(&self) -> Vec<FilterAttributeDefinition> {
        self.attributes.clone()
    }

}