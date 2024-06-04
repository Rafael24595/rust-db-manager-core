use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, PartialEq, EnumIter)]
pub enum EFilterCategory {
    ID_STRING,
    ID_NUMERIC,
    QUERY,
    STRING,
    BOOLEAN,
    NUMERIC,
    COLLECTION,
    ROOT,
}

impl EFilterCategory {
    
    pub fn root_category() -> EFilterCategory {
        EFilterCategory::ROOT
    }

    pub fn items() -> Vec<EFilterCategory> {
        EFilterCategory::iter().collect()
    }

    pub fn to_string(&self) -> String {
        match self {
            EFilterCategory::ID_STRING => String::from("ID_STRING"),
            EFilterCategory::ID_NUMERIC => String::from("ID_NUMERIC"),
            EFilterCategory::QUERY => String::from("QUERY"),
            EFilterCategory::STRING => String::from("STRING"),
            EFilterCategory::BOOLEAN => String::from("BOOLEAN"),
            EFilterCategory::NUMERIC => String::from("NUMERIC"),
            EFilterCategory::COLLECTION => String::from("COLLECTION"),
            EFilterCategory::ROOT => String::from("ROOT"),
        }
    }

    pub fn from_string(code: &str) -> Option<EFilterCategory> {
        match code {
            "IDSTRING" => Some(EFilterCategory::ID_STRING),
            "IDNUMERIC" => Some(EFilterCategory::ID_NUMERIC),
            "QUERY" => Some(EFilterCategory::QUERY),
            "STRING" => Some(EFilterCategory::STRING),
            "BOOLEAN" => Some(EFilterCategory::BOOLEAN),
            "NUMERIC" => Some(EFilterCategory::NUMERIC),
            "COLLECTION" => Some(EFilterCategory::COLLECTION),
            "ROOT" => Some(EFilterCategory::ROOT),
            _ => None
        }
    }

}