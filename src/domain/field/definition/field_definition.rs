use serde::Deserialize;

use crate::domain::field::e_field_code::EFieldCode;

use super::field_attribute_definition::FieldAttributeDefinition;

#[derive(Clone, Deserialize)]
pub struct FieldDefinition {
    order: usize,
    name: String,
    code: EFieldCode,
    swsize: bool,
    multiple: bool,
    attributes: Vec<FieldAttributeDefinition>
}

impl FieldDefinition {
    
    pub fn new(order: usize, name: String, code: EFieldCode, swsize: bool, multiple: bool, attributes: Vec<FieldAttributeDefinition>) -> Self {
        Self {
            order, name, code,
            swsize, multiple,
            attributes
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn code(&self) -> EFieldCode {
        self.code.clone()
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