#[derive(Clone)]
pub struct GenerateDatabaseQuery {
    data_base: String
}

impl GenerateDatabaseQuery {

    pub fn new(data_base: String) -> GenerateDatabaseQuery {
        GenerateDatabaseQuery {
            data_base: data_base
        }
    }

    pub fn data_base(&self) -> &str {
        &self.data_base
    }

}