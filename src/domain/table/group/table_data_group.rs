use super::{e_data_type::EDataType, table_data_field::TableDataField};

#[derive(Debug, Clone)]
pub struct TableDataGroup {
    order: usize,
    name: String,
    fields: Vec<TableDataField>,
}

impl TableDataGroup {
    
    pub fn new(order: usize, name: String) -> Self {
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

    pub fn fields(&self) -> Vec<TableDataField> {
        self.fields.clone()
    }

    pub fn push(&mut self, key: String, value: String) -> &mut Self {
        self.fields.push(
            TableDataField::new(self.fields.len(), key, value)
        );
        self
    }

    pub fn push_typed(&mut self, key: String, value: String, data_type: EDataType) -> &mut Self {
        self.fields.push(
            TableDataField::new_typed(self.fields.len(), key, value, data_type)
        );
        self
    }

}