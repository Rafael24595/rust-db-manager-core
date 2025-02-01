use serde::Deserialize;

use crate::domain::field::{definition::{field_attribute_definition::FieldAttributeDefinition, field_definition::FieldDefinition}, generate::field_data::FieldData};

use super::collections_reference_definition::CollectionReferenceDefinition;

#[derive(Clone, Deserialize)]
pub struct CollectionDefinition {
    swrelational: bool,
    definition: Vec<FieldDefinition>,
    defaults: Vec<FieldData>,
    global_attributes: Vec<FieldAttributeDefinition>,
    references: Vec<CollectionReferenceDefinition>
}

impl CollectionDefinition {
    
    pub fn new(swrelational: bool, definition: Vec<FieldDefinition>, defaults: Vec<FieldData>, global_attributes: Vec<FieldAttributeDefinition>, references: Vec<CollectionReferenceDefinition>) -> Self {
        Self {
            swrelational, definition, defaults, global_attributes, references
        }
    }

    pub fn is_relational(&self) -> bool {
        self.swrelational
    }

    pub fn definition(&self) -> &Vec<FieldDefinition> {
        &self.definition
    }

    pub fn defaults(&self) -> &Vec<FieldData> {
        &self.defaults
    }

    pub fn global_attributes(&self) -> &Vec<FieldAttributeDefinition> {
        &self.global_attributes
    }

    pub fn references(&self) -> &Vec<CollectionReferenceDefinition> {
        &self.references
    }

    pub fn push_references(&mut self, mut references: Vec<CollectionReferenceDefinition>) -> &Self {
        self.references.append(&mut references);
        self
    }

}