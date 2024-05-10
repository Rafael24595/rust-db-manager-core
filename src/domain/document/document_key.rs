use super::document_key_attribute::DocumentKeyAttribute;

#[derive(Debug, Clone)]
pub struct DocumentKey {
    name: String,
    value: String,
    attributes: Vec<DocumentKeyAttribute>
}

impl DocumentKey {
    
    pub fn new(name: String, value: String, attributes: Vec<DocumentKeyAttribute>) -> Self {
        Self {
            name, value, attributes
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn attributes(&self) -> Vec<DocumentKeyAttribute> {
        self.attributes.clone()
    }

}