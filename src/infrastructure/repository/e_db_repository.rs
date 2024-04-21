#[derive(Clone)]
pub enum EDBRepository {
    MongoDB
}

impl EDBRepository {

    pub fn to_string(&self) -> String {
        match self {
            EDBRepository::MongoDB => String::from("MongoDB")
        }
    }

    pub fn from_string(category: String) -> Option<EDBRepository> {
        match category.as_str() {
            "MongoDB" => Some(EDBRepository::MongoDB),
            _ => None,
        }
    }

}