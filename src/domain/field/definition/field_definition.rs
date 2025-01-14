use serde::Deserialize;

use super::field_attribute_definition::FieldAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FieldDefinition {
    order: usize,
    name: String,
    code: String,
    swkey: bool,
    swsize: bool,
    multiple: bool,
    attributes: Vec<FieldAttributeDefinition>
}

impl FieldDefinition {
    
    pub fn new(order: usize, name: String, code: String, swkey: bool, swsize: bool, multiple: bool, attributes: Vec<FieldAttributeDefinition>) -> Self {
        Self {
            order, name, code,
            swkey, swsize, multiple,
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

    pub fn can_key(&self) -> bool {
        self.swkey
    }

    pub fn has_size(&self) -> bool {
        self.swsize
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn attributes(&self) -> Vec<FieldAttributeDefinition> {
        self.attributes.clone()
    }

}