use serde::Deserialize;

use super::action_form::ActionForm;

#[derive(Clone, Deserialize)]
pub struct ActionFormCollection {
    sw_query: bool,
    forms: Vec<ActionForm>
}

impl ActionFormCollection {
    
    pub fn new(sw_query: bool) -> Self {
        Self {
            sw_query: sw_query,
            forms: Vec::new()
        }
    }

    pub fn is_query(&self) -> bool {
        self.sw_query
    }

    pub fn forms(&self) -> Vec<ActionForm> {
        self.forms.clone()
    }

    pub fn push(& mut self, form: ActionForm) -> &Self {
        self.forms.push(form);
        self
    }

}