use serde::Deserialize;

use super::table_field_definition::TableFieldDefinition;

#[derive(Debug, Clone, Deserialize)]
pub struct TableRowDefinition {
    fields: Vec<TableFieldDefinition>
}

impl TableRowDefinition {
    
    pub fn new() -> Self {
        Self {
            fields: Vec::new()
        }
    }

    pub fn fields(&self) -> Vec<TableFieldDefinition>{
        self.fields.clone()
    }

    pub fn push_title(& mut self, value: String) -> &Self {
        let field = TableFieldDefinition::new(value, true);
        self.fields.push(field);
        self
    }

    pub fn push(& mut self, value: String) -> &Self {
        let field = TableFieldDefinition::new(value, false);
        self.fields.push(field);
        self
    }

}