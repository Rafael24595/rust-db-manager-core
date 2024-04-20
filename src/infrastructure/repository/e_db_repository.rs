#[derive(Clone)]
pub enum EDBRepository {
    MongoDB
}

impl ToString for EDBRepository {
    fn to_string(&self) -> String {
        match self {
            EDBRepository::MongoDB => String::from("MongoDB")
        }
    }
}