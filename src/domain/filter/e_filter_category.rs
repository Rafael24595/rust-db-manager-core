#[derive(Clone, PartialEq)]
pub enum EFilterCategory {
    IDSTRING,
    IDNUMERIC,
    QUERY,
    STRING,
    BOOLEAN,
    NUMERIC,
    COLLECTION,
    ROOT,
}

impl EFilterCategory {
    
    pub fn from_string(code: &str) -> Option<EFilterCategory> {
        match code {
            "IDSTRING" => Some(EFilterCategory::IDSTRING),
            "IDNUMERIC" => Some(EFilterCategory::IDNUMERIC),
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