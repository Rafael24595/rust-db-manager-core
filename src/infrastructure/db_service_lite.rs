use super::repository::e_db_repository::EDBRepository;

#[derive(Clone)]
pub struct DBServiceLite {
    name: String,
    category: EDBRepository,
}

impl DBServiceLite {

    pub fn new(name: String, category: EDBRepository) -> DBServiceLite {        
        DBServiceLite {
            name, category
        }
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn category(&self) -> EDBRepository {
        self.category.clone()
    }

}