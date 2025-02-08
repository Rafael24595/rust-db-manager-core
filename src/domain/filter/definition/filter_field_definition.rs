use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FilterFieldDefinition {
    field: String,
    json_type: String,
    defaults: Vec<String>
}

impl FilterFieldDefinition {

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn json_type(&self) -> &str {
        &self.json_type
    }

    pub fn defaults(&self) -> &Vec<String> {
        &self.defaults
    }

}