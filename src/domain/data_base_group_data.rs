use super::data_base_field::DataBaseField;

#[derive(Debug, Clone)]
pub struct DataBaseDataGroup {
    order: usize,
    name: String,
    fields: Vec<DataBaseField>,
}

impl DataBaseDataGroup {
    
    pub fn new(order: usize, name: String) -> DataBaseDataGroup {
        Self {
            order: order,
            name: name,
            fields: Vec::new()
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn fields(&self) -> Vec<DataBaseField> {
        self.fields.clone()
    }

    pub fn push(&mut self, key: String, value: String) -> &mut Self {
        self.fields.push(
            DataBaseField::new(self.fields.len(), key, value)
        );
        self
    }

}