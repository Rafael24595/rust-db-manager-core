#[derive(Debug, Clone, PartialEq)]
pub enum EJSONType {
    STRING,
    BOOLEAN,
    NUMERIC
}

impl EJSONType {
    
    pub fn to_string(&self) -> String {
        match self {
            EJSONType::STRING => String::from("STRING"),
            EJSONType::BOOLEAN => String::from("BOOLEAN"),
            EJSONType::NUMERIC => String::from("NUMERIC"),
        }
    }

    pub fn from_string(code: &str) -> Option<EJSONType> {
        match code {
            "STRING" => Some(EJSONType::STRING),
            "BOOLEAN" => Some(EJSONType::BOOLEAN),
            "NUMERIC" => Some(EJSONType::NUMERIC),
            _ => None
        }
    }

}