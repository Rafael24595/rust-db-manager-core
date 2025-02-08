use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FilterDefinitionQuery {
    category: String,
    json_type: String,
    example: String
}

impl FilterDefinitionQuery {

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn json_type(&self) -> &str {
        &self.json_type
    }

    pub fn example(&self) -> &str {
        &self.example
    }

}