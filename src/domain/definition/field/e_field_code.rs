use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EFieldCode {
    INDEXED
}

impl EFieldCode {
    
    pub fn to_string(&self) -> String {
        match self {
            EFieldCode::INDEXED => String::from("INDEXED")
        }
    }

    pub fn from_string(code: &str) -> Option<EFieldCode> {
        match code {
            "INDEXED" => Some(EFieldCode::INDEXED),
            _ => None
        }
    }


}