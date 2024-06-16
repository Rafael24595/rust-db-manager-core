use serde::Deserialize;

use super::form_field_definition::FormFieldDefinition;

#[derive(Clone, Deserialize)]
pub struct ActionForm {
    title: Option<String>,
    sw_vector: bool,
    fields: Vec<FormFieldDefinition>
}

impl ActionForm {
    
    pub fn new(title: Option<String>, sw_vector: bool) -> Self {
        Self {
            title: title,
            sw_vector: sw_vector,
            fields: Vec::new()
        }
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn is_vector(&self) -> bool {
        self.sw_vector
    }

    pub fn fields(&self) -> Vec<FormFieldDefinition> {
        self.fields.clone()
    }

    pub fn push(& mut self, field: FormFieldDefinition) -> &Self {
        self.fields.push(field);
        self
    }

}