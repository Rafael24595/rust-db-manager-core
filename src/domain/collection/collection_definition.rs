use serde::Deserialize;

use crate::domain::field::{definition::field_definition::FieldDefinition, generate::field_data::FieldData};

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