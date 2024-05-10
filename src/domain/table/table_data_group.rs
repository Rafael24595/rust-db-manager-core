use super::table_data_field::TableDataField;

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

}