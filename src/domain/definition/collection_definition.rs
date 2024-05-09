use serde::Deserialize;

use crate::domain::generate::field::field_data::FieldData;

use super::field::field_definition::FieldDefinition;

#[derive(Clone, Deserialize)]
pub struct CollectionDefinition {
    swrelational: bool,
    definition: Vec<FieldDefinition>,
    defaults: Vec<FieldData>
}

impl CollectionDefinition {
    
    pub fn new(swrelational: bool, definition: Vec<FieldDefinition>, defaults: Vec<FieldData>) -> Self {
        Self {
            swrelational, definition, defaults
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

}