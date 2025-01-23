use serde::Deserialize;

use crate::domain::field::{definition::{field_attribute_definition::FieldAttributeDefinition, field_definition::FieldDefinition}, generate::field_data::FieldData};

#[derive(Clone, Deserialize)]
pub struct CollectionDefinition {
    swrelational: bool,
    definition: Vec<FieldDefinition>,
    defaults: Vec<FieldData>,
    global_attributes: Vec<FieldAttributeDefinition>
}

impl CollectionDefinition {
    
    pub fn new(swrelational: bool, definition: Vec<FieldDefinition>, defaults: Vec<FieldData>, global_attributes: Vec<FieldAttributeDefinition>) -> Self {
        Self {
            swrelational, definition, defaults, global_attributes
        }
    }

    pub fn is_relational(&self) -> bool {
        self.swrelational
    }

    pub fn definition(&self) -> Vec<FieldDefinition> {
        self.definition.clone()
    }

    pub fn defaults(&self) -> Vec<FieldData> {
        self.defaults.clone()
    }

    pub fn global_attributes(&self) -> &Vec<FieldAttributeDefinition> {
        &self.global_attributes
    }

}