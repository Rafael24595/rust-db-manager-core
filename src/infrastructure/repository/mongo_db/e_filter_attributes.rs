pub enum EFilterAtributtes {
    OID,
    REGEX
}

impl EFilterAtributtes {
    
    pub fn to_string(&self) -> String {
        match self {
            EFilterAtributtes::OID => String::from("OID"),
            EFilterAtributtes::REGEX => String::from("REGEX"),
        }
    }

}