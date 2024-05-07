#[derive(Clone, PartialEq)]
pub enum EFieldCode {
    ID
}

impl EFieldCode {
    
    pub fn to_string(&self) -> String {
        match self {
            EFieldCode::ID => String::from("ID")
        }
    }

}