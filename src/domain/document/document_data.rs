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

    pub fn format(&self) -> EDocumentFormat {
        self.format.clone()
    }

    pub fn data_base(&self) -> String {
        self.data_base.clone()
    }

    pub fn collection(&self) -> String {
        self.collection.clone()
    }

    pub fn document(&self) -> String {
        self.document.clone()
    }

}