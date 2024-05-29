use crate::domain::field::generate::field_data::FieldData;

pub struct DocumentSchema {
    comments: Vec<String>,
    sw_strict: bool,
    fields: Vec<FieldData>
}

impl DocumentSchema {
    
    pub fn new(comments: Vec<String>, sw_strict: bool, fields: Vec<FieldData>) -> Self {
        Self {
            comments, sw_strict, fields
        }
    }

    pub fn comments(&self) -> Vec<String> {
        self.comments.clone()
    }

    pub fn is_strict(&self) -> bool {
        self.sw_strict
    }

    pub fn fields(&self) -> Vec<FieldData> {
        self.fields.clone()
    }

}