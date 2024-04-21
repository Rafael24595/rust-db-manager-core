use std::time::{SystemTime, UNIX_EPOCH};

use crate::{commons::exception::connect_exception::ConnectException, domain::connection_data::ConnectionData, infrastructure::repository::{db_dictionary, i_db_repository::IDBRepository}, service::service::Service};

#[derive(Clone)]
pub struct DBService {
    name: String,
    owner: String,
    timestamp: u128,
    connection_data: ConnectionData
}

impl DBService {

    pub fn new(name: String, owner: String, connection_data: ConnectionData) -> DBService {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cannot read actual date.")
            .as_millis();
        
        DBService {
            name, owner, timestamp, connection_data
        }
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub async fn instance(&self) -> Result<Service<impl IDBRepository>, ConnectException> {
        let repository = db_dictionary::find(self.connection_data.clone()).await?;
        Ok(Service::from(repository))
    }

}