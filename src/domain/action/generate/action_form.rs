use super::form_field::FormField;

#[derive(Clone)]
pub struct ActionForm {
    code: String,
    fields: Vec<Vec<FormField>>
}

impl ActionForm {
    
    pub fn new(code: String, fields: Vec<Vec<FormField>>) -> Self {
        Self {
            code: code,
            fields: fields
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn fields(&self) -> &Vec<Vec<FormField>> {
        &self.fields
    }

    pub fn find_fields(&self, code: String) -> Vec<&FormField> {
        self.fields.iter()
            .flat_map(|form| form.iter())
            .filter(|&field| field.code() == code)
            .collect()
    }

}