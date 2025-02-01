use crate::domain::e_json_type::EJSONType;

use super::document_key_attribute::DocumentKeyAttribute;

#[derive(Debug, Clone)]
pub struct DocumentKey {
    name: String,
    value: String,
    json_type: EJSONType,
    attributes: Vec<DocumentKeyAttribute>
}

impl DocumentKey {
    
    pub fn new(name: String, value: String, json_type: EJSONType, attributes: Vec<DocumentKeyAttribute>) -> Self {
        Self {
            name, value, json_type, attributes
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn json_type(&self) -> &EJSONType {
        &self.json_type
    }

    pub fn attributes(&self) -> &Vec<DocumentKeyAttribute> {
        &self.attributes
    }

}