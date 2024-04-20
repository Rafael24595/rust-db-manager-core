use super::e_collection_field::ECollectionField;

pub struct CollectionField {
    collection: String,
    name: String,
    category: ECollectionField,
    sw_key: bool,
    length: i8,
    reference: Vec<CollectionField>
}

impl CollectionField {
    
    pub fn new(collection: String, name: String, category: ECollectionField, length: i8) -> CollectionField {
        CollectionField {
            collection: collection,
            name: name,
            category: category,
            sw_key: false,
            length: length,
            reference: Vec::new()
        }
    }

    pub fn new_key(collection: String, name: String, category: ECollectionField, length: i8) -> CollectionField {
        CollectionField {
            collection: collection,
            name: name,
            category: category,
            sw_key: true,
            length: length,
            reference: Vec::new()
        }
    }

    pub fn push(&mut self, reference: CollectionField) -> &Self {
        self.reference.push(reference);
        self
    }

}