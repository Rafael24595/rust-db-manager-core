use std::sync::{Arc};

use tokio::sync::Mutex;

use crate::{commons::exception::connect_exception::ConnectException, infrastructure::db_service::DBService, service::service::Service};

pub struct DBConnection {
    configuration: DBService,
    connection: Option<Arc<Mutex<Service>>>
}

impl DBConnection {

    pub fn new(configuration: &DBService) -> DBConnection {
        Self { 
            configuration: configuration.clone(), 
            connection: None
        }
    }
    
    pub fn configuration(&self) -> &DBService {
        &self.configuration
    }

    pub async fn connection(&mut self) -> Result<Arc<Mutex<Service>>, ConnectException> {
        if self.connection.is_none() {
            let conn = self.configuration.instance().await?;
            self.connection = Some(Arc::new(Mutex::new(conn)));
        }
        Ok(Arc::clone(self.connection.as_ref().unwrap()))
    }

}