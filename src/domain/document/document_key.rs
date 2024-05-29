use crate::domain::e_json_type::EJSONType;

use super::document_key_attribute::DocumentKeyAttribute;

#[derive(Debug, Clone)]
pub struct DocumentKey {
    name: String,
    value: String,
    jtype: EJSONType,
    attributes: Vec<DocumentKeyAttribute>
}

impl DocumentKey {
    
    pub fn new(name: String, value: String, jtype: EJSONType, attributes: Vec<DocumentKeyAttribute>) -> Self {
        Self {
            name, value, jtype, attributes
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn jtype(&self) -> EJSONType {
        self.jtype.clone()
    }

    pub fn attributes(&self) -> Vec<DocumentKeyAttribute> {
        self.attributes.clone()
    }

}