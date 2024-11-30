use super::e_data_type::EDataType;

#[derive(Debug, Clone)]
pub struct TableDataField {
    order: usize,
    name: String,
    value: String,
    data_type: EDataType,
    json_type: String,
}

impl TableDataField {

    pub fn new(order: usize, name: String, value: String) -> Self {
        Self::new_typed(order, name, value, EDataType::STRING)
    }
    
    pub fn new_typed(order: usize, name: String, value: String, data_type: EDataType) -> Self {
        Self {
            order: order,
            name: name,
            value: value,
            data_type: data_type,
            json_type: String::from("string")
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn data_type(&self) -> EDataType {
        self.data_type.clone()
    }
    
    pub fn json_type(&self) -> String {
        self.json_type.clone()
    }

}