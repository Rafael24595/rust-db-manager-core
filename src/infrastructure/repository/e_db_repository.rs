use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, EnumIter)]
pub enum EDBRepository {
    MongoDB
}

impl EDBRepository {

    pub fn items() -> Vec<EDBRepository> {
        EDBRepository::iter().collect()
    }

    pub fn to_string(&self) -> String {
        match self {
            EDBRepository::MongoDB => String::from("MongoDB")
        }
    }

    pub fn from_string(category: &str) -> Option<EDBRepository> {
        match category {
            "MongoDB" => Some(EDBRepository::MongoDB),
            _ => None,
        }
    }

}