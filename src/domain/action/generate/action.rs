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

    pub fn action(&self) -> String {
        self.action.clone()
    }

    pub fn form(&self) -> Vec<ActionForm> {
        self.form.clone()
    }

    pub fn find_form(&self, code: String) -> Option<&ActionForm> {
        self.form.iter().find(|f| f.code() == code)
    }

}