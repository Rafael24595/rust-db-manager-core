use serde::Deserialize;

use super::filter_field_definition::FilterFieldDefinition;

#[derive(Clone, Deserialize)]
pub struct FilterFieldsDefinition {
    category: String,
    fields: Vec<FilterFieldDefinition>
}

impl FilterFieldsDefinition {

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn fields(&self) -> &Vec<FilterFieldDefinition> {
        &self.fields
    }

}