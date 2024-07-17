use serde::Deserialize;

use crate::domain::table::definition::table_definition::TableDefinition;

use super::action_form_collection::ActionFormCollection;

#[derive(Clone, Deserialize)]
pub struct ActionDefinition {
    action: String,
    title: String,
    data: Option<Vec<TableDefinition>>,
    form: Option<ActionFormCollection>
}

impl ActionDefinition {
    
    pub fn new(action: String, title: String, data: Option<Vec<TableDefinition>>, form: Option<ActionFormCollection>) -> Self {
        Self {
            action, title, data, form
        }
    }

    pub fn action(&self) -> String {
        self.action.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn data(&self) -> Option<Vec<TableDefinition>> {
        self.data.clone()
    }

    pub fn form(&self) -> Option<ActionFormCollection> {
        self.form.clone()
    }

}