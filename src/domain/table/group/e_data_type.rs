use strum::EnumIter;

#[derive(Clone, Debug, PartialEq, EnumIter)]
pub enum EDataType {
    STRING,
    BYTE,
    TIMESTAMP
}

impl EDataType {

    pub fn to_string(&self) -> String {
        match self {
            EDataType::STRING => String::from("STRING"),
            EDataType::BYTE => String::from("BYTE"),
            EDataType::TIMESTAMP => String::from("TIMESTAMP"),
        }
    }

    pub fn from_string(code: &str) -> Option<EDataType> {
        match code {
            "STRING" => Some(EDataType::STRING),
            "BYTE" => Some(EDataType::BYTE),
            "TIMESTAMP" => Some(EDataType::TIMESTAMP),
            _ => None
        }
    }

}