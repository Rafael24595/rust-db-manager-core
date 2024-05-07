#[derive(Clone, PartialEq)]
pub enum EFieldCategory {
    NUMBER,
    STRING,
    BOOLEAN
}

impl EFieldCategory {
    
    pub fn to_string(&self) -> String {
        match self {
            EFieldCategory::NUMBER => String::from("NUMBER"),
            EFieldCategory::STRING => String::from("STRING"),
            EFieldCategory::BOOLEAN => String::from("BOOLEAN"),
        }
    }

}