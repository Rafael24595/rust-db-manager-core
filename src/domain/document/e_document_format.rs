#[derive(Debug, Clone)]
pub enum EDocumentFormat {
    JSON,
    TABLE
}

impl EDocumentFormat {

    pub fn to_string(&self) -> String {
        match self {
            EDocumentFormat::JSON => String::from("JSON"),
            EDocumentFormat::TABLE => String::from("TABLE"),
        }
    }

    pub fn from_string(code: &str) -> Option<EDocumentFormat> {
        match code {
            "JSON" => Some(EDocumentFormat::JSON),
            "TABLE" => Some(EDocumentFormat::TABLE),
            _ => None
        }
    }

}