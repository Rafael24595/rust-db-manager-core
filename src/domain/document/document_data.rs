use super::e_document_format::EDocumentFormat;

#[derive(Debug, Clone)]
pub struct DocumentData {
    format: EDocumentFormat,
    data_base: String,
    collection: String,
    document: String
}

impl DocumentData {
    
    pub fn new(format: EDocumentFormat, data_base: String, collection: String, document: String) -> Self {
        Self {
            format, data_base, collection, document
        }
    }

    pub fn format(&self) -> &EDocumentFormat {
        &self.format
    }

    pub fn data_base(&self) -> &str {
        &self.data_base
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn document(&self) -> &str {
        &self.document
    }

}