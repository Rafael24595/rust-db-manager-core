use serde::Deserialize;

use super::table_row_definition::TableRowDefinition;

#[derive(Debug, Clone, Deserialize)]
pub struct TableDefinition {
    title: String,
    rows: Vec<TableRowDefinition>,
}

impl TableDefinition {

    pub fn new(title: String) -> Self {
        Self {
            title: title,
            rows: Vec::new()
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn rows(&self) -> &Vec<TableRowDefinition> {
        &self.rows
    }

    pub fn push(& mut self, row: TableRowDefinition) -> &Self {
        self.rows.push(row);
        self
    }

}