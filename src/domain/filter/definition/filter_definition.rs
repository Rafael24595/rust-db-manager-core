use serde::Deserialize;

use super::{filter_attribute_definition::FilterAttributeDefinition, filter_definition_query::FilterDefinitionQuery, filter_fields_definition::FilterFieldsDefinition};

#[derive(Clone, Deserialize)]
pub struct FilterDefinition {
    category_root: String,
    category_query: FilterDefinitionQuery,
    categories: Vec<String>,
    fields: Vec<FilterFieldsDefinition>,
    attributes: Vec<FilterAttributeDefinition>
}

impl FilterDefinition {

    pub fn category_root(&self) -> &str {
        &self.category_root
    }

    pub fn category_query(&self) -> &FilterDefinitionQuery {
        &self.category_query
    }

    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }

    pub fn fields(&self) -> &Vec<FilterFieldsDefinition> {
        &self.fields
    }

    pub fn attributes(&self) -> &Vec<FilterAttributeDefinition> {
        &self.attributes
    }

    pub fn push_field(&mut self, other: FilterFieldsDefinition) -> &Self {
        self.fields.push(other);
        self
    }

    pub fn append_fields(&mut self, mut others: Vec<FilterFieldsDefinition>) -> &Self {
        self.fields.append(&mut others);
        self
    }

}