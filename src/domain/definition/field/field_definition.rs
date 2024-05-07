use super::{e_field_category::EFieldCategory, e_field_code::EFieldCode, field_attribute_definition::FieldAttributeDefinition};

#[derive(Clone)]
pub struct FieldDefinition {
    order: usize,
    name: String,
    code: EFieldCode,
    category: EFieldCategory,
    size: bool,
    multiple: bool,
    attributes: Vec<FieldAttributeDefinition>
}

impl FieldDefinition {
    
    pub fn new(order: usize, name: String, code: EFieldCode, category: EFieldCategory, size: bool, multiple: bool, attributes: Vec<FieldAttributeDefinition>) -> Self {
        Self {
            order, name, code,
            category, size, multiple,
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

    pub fn category(&self) -> EFieldCategory {
        self.category.clone()
    }

    pub fn size(&self) -> bool {
        self.size
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn attributes(&self) -> Vec<FieldAttributeDefinition> {
        self.attributes.clone()
    }

}