use serde::{Deserialize, Serialize};

use crate::infrastructure::repository::e_db_repository::EDBRepository;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectionData {
    category: EDBRepository,
    connection: String
}

impl ConnectionData {

    pub fn new(category: EDBRepository, connection: String) -> ConnectionData {
        ConnectionData {
            category,
            connection
        }
    }

    pub fn category(&self) -> EDBRepository {
        return self.category.clone();
    }

    pub fn connection(&self) -> String {
        return self.connection.clone();
    }

}