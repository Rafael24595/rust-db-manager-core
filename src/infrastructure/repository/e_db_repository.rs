use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, EnumIter, Deserialize, Serialize)]
pub enum EDBRepository {
    MongoDB,
    Postgres,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EDBRepositoryItem {
    pub item: EDBRepository,
    pub default: bool
}

impl EDBRepository {

    pub fn items() -> Vec<EDBRepositoryItem> {
        let mut items = Vec::new();
        items.push(EDBRepositoryItem{item: EDBRepository::MongoDB, default: false});
        items.push(EDBRepositoryItem{item: EDBRepository::Postgres, default: false});
        return items;
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