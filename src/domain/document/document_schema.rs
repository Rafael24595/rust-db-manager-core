use crate::domain::field::generate::field_data::FieldData;

pub struct DocumentSchema {
    comments: Vec<String>,
    fields: Vec<FieldData>
}

impl DocumentSchema {
    
    pub fn new(comments: Vec<String>, fields: Vec<FieldData>) -> Self {
        Self {
            comments, fields
        }
    }

    pub fn comments(&self) -> Vec<String> {
        self.comments.clone()
    }

    pub fn fields(&self) -> Vec<FieldData> {
        self.fields.clone()
    }

}