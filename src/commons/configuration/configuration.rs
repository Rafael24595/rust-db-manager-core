use std::{collections::HashMap, sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

use lazy_static::lazy_static;
use uuid::Uuid;

use crate::infrastructure::db_service::DBService;

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    session_id: String,
    timestamp: u128,
    services: HashMap<String, DBService>
}

impl Configuration {
    
    pub fn initialize() -> Configuration {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_some() {
            //TODO: Log.
            panic!("Configuration is already initialized.");
        }

        let session_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cannot read actual date.")
            .as_millis();
        let services = HashMap::new();

        let config = Configuration {
            session_id, timestamp, services
        };

        *instance = Some(config);
        
        return instance.as_ref().unwrap().clone();
    }

    pub fn instance() -> Configuration {
        let instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_none() {
            //TODO: Log.
            panic!("Configuration is not initialized.");
        }
        
        return instance.as_ref().unwrap().clone();
    }

    pub fn session_id() -> String {
        Configuration::instance().session_id
    }

    pub fn timestamp() -> u128 {
        Configuration::instance().timestamp
    }

    pub fn push_service(key: String, service: DBService) -> Configuration {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        config.services.insert(key, service);
        
        return config.clone();
    }

    pub fn find_service(key: String) -> Option<DBService> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        config.services.get(&key).cloned()
    }

}