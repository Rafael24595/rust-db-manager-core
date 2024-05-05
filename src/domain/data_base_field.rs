#[derive(Debug, Clone)]
pub struct DataBaseField {
    order: usize,
    name: String,
    value: String,
    json_type: String,
}

impl DataBaseField {
    
    pub fn new(order: usize, name: String, value: String) -> DataBaseField {
        Self {
            order: order,
            name: name,
            value: value,
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
    
    pub fn json_type(&self) -> String {
        self.json_type.clone()
    }

}