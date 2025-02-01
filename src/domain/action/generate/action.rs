use super::action_form::ActionForm;

#[derive(Clone)]
pub struct Action {
    action: String,
    form: Vec<ActionForm>
}

impl Action {
    
    pub fn new(action: String, form: Vec<ActionForm>) -> Self {
        Self {
            action, form
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn form(&self) -> &Vec<ActionForm> {
        &self.form
    }

    pub fn find_form(&self, code: String) -> Option<&ActionForm> {
        self.form.iter().find(|f| f.code() == code)
    }

}