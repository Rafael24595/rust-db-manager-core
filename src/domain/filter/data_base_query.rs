#[derive(Clone)]
pub struct DataBaseQuery {
    data_base: String,
}

impl DataBaseQuery {
    
    pub fn from(data_base: String) -> Self {
        Self {
            data_base
        }
    }

    pub fn data_base(&self) -> &str {
        &self.data_base
    }

}