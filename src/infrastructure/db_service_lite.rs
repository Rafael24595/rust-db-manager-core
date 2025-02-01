use super::repository::e_db_repository::EDBRepository;

#[derive(Clone)]
pub struct DBServiceLite {
    name: String,
    protected: bool,
    category: EDBRepository,
}

impl DBServiceLite {

    pub fn new(name: String, protected: bool, category: EDBRepository) -> DBServiceLite {        
        DBServiceLite {
            name, protected, category
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_protected(&self) -> bool {
        self.protected
    }

    pub fn category(&self) -> &EDBRepository {
        &self.category
    }

}