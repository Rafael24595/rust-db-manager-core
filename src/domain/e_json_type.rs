use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum EJSONType {
    STRING,
    BOOLEAN,
    NUMERIC,
    OBJECT,
    ARRAY
}

impl EJSONType {
    
    pub fn to_string(&self) -> String {
        match self {
            EJSONType::STRING => String::from("STRING"),
            EJSONType::BOOLEAN => String::from("BOOLEAN"),
            EJSONType::NUMERIC => String::from("NUMERIC"),
            EJSONType::OBJECT => String::from("OBJECT"),
            EJSONType::ARRAY => String::from("ARRAY"),
        }
    }

    pub fn from_string(code: &str) -> Option<EJSONType> {
        match code {
            "STRING" => Some(EJSONType::STRING),
            "BOOLEAN" => Some(EJSONType::BOOLEAN),
            "NUMERIC" => Some(EJSONType::NUMERIC),
            "OBJECT" => Some(EJSONType::OBJECT),
            "ARRAY" => Some(EJSONType::ARRAY),
            _ => None
        }
    }

}