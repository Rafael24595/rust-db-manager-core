use super::field::field_data::FieldData;

#[derive(Clone)]
pub struct GenerateCollectionQuery {
    data_base: String,
    collection: String,
    fields: Vec<FieldData>
}

impl GenerateCollectionQuery {

    pub fn from_data_base(data_base: String) -> Self {
        Self {
            data_base: data_base,
            collection: String::new(),
            fields: Vec::new()
        }
    }

    pub fn from_collection(data_base: String, collection: String) -> Self {
        Self {
            data_base: data_base,
            collection: collection,
            fields: Vec::new()
        }
    }

    pub fn new(data_base: String, collection: String, fields: Vec<FieldData>) -> Self {
        Self {
            data_base: data_base,
            collection: collection,
            fields: fields
        }
    }

    pub fn data_base(&self) -> String {
        return self.data_base.clone();
    }

    pub fn collection(&self) -> String {
        return self.collection.clone();
    }

    pub fn fields(&self) -> Vec<FieldData> {
        return self.fields.clone();
    }

}