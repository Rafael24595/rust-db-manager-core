use serde::Deserialize;

use super::{field_attribute::FieldAttribute, field_reference::FieldReference};

#[derive(Debug, Clone, Deserialize)]
pub struct FieldData {
    order: i32,
    code: String,
    value: String,
    swsize: bool,
    size: i32,
    mutable: bool,
    attributes: Vec<FieldAttribute>,
    reference: Vec<FieldReference>
}

impl FieldData {
 
    pub fn new(order: i32, code: String, value: String, swsize: bool, size: i32, mutable: bool, attributes: Vec<FieldAttribute>, reference: Vec<FieldReference>) -> Self {
        Self { 
            order, code, value, 
            swsize, size, mutable, 
            attributes, reference 
        }
    }

    pub fn order(&self) -> i32 {
        self.order
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_resize(&self) -> bool {
        self.swsize
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn attributes(&self) -> Vec<FieldAttribute> {
        self.attributes.clone()
    }

    pub fn reference(&self) -> Vec<FieldReference> {
        self.reference.clone()
    }

}