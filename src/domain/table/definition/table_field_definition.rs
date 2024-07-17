use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TableFieldDefinition {
    data: String,
    sw_title: bool
}

impl TableFieldDefinition {
    
    pub fn new(data: String, sw_title: bool) -> Self {
        Self {
            data, sw_title
        }
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn is_title(&self) -> bool {
        self.sw_title
    }

}