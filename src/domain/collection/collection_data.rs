use crate::domain::document::document_data::DocumentData;

pub struct CollectionData {
    total: usize,
    limit: Option<usize>,
    offset: Option<usize>,
    documents: Vec<DocumentData>
}

impl CollectionData {
    
    pub fn new(total: usize, limit: Option<usize>, offset: Option<usize>, documents: Vec<DocumentData>) -> Self {
        Self {
            total, limit, offset, documents
        }
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn offset(&self) -> Option<usize> {
        self.offset
    }

    pub fn documents(&self) -> Vec<DocumentData> {
        self.documents.clone()
    }

}