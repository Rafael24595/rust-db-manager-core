use serde::Deserialize;

use super::field_attribute_definition::FieldAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FieldDefinition {
    order: usize,
    name: String,
    code: String,
    swsize: bool,
    multiple: bool,
    attributes: Vec<FieldAttributeDefinition>
}

impl FieldDefinition {
    
    pub fn new(order: usize, name: String, code: String, swsize: bool, multiple: bool, attributes: Vec<FieldAttributeDefinition>) -> Self {
        Self {
            order, name, code,
            swsize, multiple,
            attributes
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn swsize(&self) -> bool {
        self.swsize
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn attributes(&self) -> Vec<FieldAttributeDefinition> {
        self.attributes.clone()
    }

}