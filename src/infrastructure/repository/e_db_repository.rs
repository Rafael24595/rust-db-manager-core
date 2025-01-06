use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, EnumIter, Deserialize, Serialize)]
pub enum EDBRepository {
    MongoDB,
    Postgres,
}

impl EDBRepository {

    pub fn items() -> Vec<EDBRepository> {
        EDBRepository::iter().collect()
    }

    pub fn to_string(&self) -> String {
        match self {
            EDBRepository::MongoDB => String::from("MongoDB"),
            EDBRepository::Postgres => String::from("Postgres"),
        }
    }

    pub fn from_string(category: &str) -> Option<EDBRepository> {
        match category {
            "MongoDB" => Some(EDBRepository::MongoDB),
            "Postgres" => Some(EDBRepository::Postgres),
            _ => None,
        }
    }

}