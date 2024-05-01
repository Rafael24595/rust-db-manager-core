use std::{collections::HashMap, process::Command, sync::Mutex, time::{SystemTime, UNIX_EPOCH}};

use cargo_metadata::{CargoOpt, MetadataCommand};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::{commons::exception::connect_exception::ConnectException, infrastructure::{db_service::DBService, db_service_lite::DBServiceLite}};

lazy_static! {
    static ref INSTANCE: Mutex<Option<Configuration>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct Configuration {
    rustc_version: String,
    cargo_version: String,
    app_name: String,
    app_version: String,
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

        let rustc_version = Configuration::command_rustc_version();
        let cargo_version = Configuration::command_cargo_version();

        let metadata = MetadataCommand::new()
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let root: &cargo_metadata::Package = metadata.packages.iter()
            .find(|i| i.name == "rust_db_manager_core").unwrap();

        let app_name = root.name.clone();
        let app_version = root.version.clone().to_string();

        let session_id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cannot read actual date.")
            .as_millis();
        let services = HashMap::new();

        let config = Configuration {
            rustc_version, cargo_version, app_name, app_version, session_id, timestamp, services
        };

        *instance = Some(config);
        
        return instance.as_ref().unwrap().clone();
    }

    fn command_cargo_version() -> String {
        Configuration::command_lang_version("cargo")
    }

    fn command_rustc_version() -> String {
        Configuration::command_lang_version("rustc")
    }

    fn command_lang_version(resource: &str) -> String {
        let output = Command::new(resource)
            .arg("--version")
            .output()
            .expect("Failed to execute command");
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).to_string();
        } else {
            //TODO: Log.
            panic!("Failed to get {} version", resource);
        }
    }

    fn instance() -> Configuration {
        let instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_none() {
            //TODO: Log.
            panic!("Configuration is not initialized.");
        }
        
        return instance.as_ref().unwrap().clone();
    }

    pub fn rustc_version() -> String {
        Configuration::instance().rustc_version
    }

    pub fn cargo_version() -> String {
        Configuration::instance().cargo_version
    }

    pub fn name() -> String {
        Configuration::instance().app_name
    }

    pub fn version() -> String {
        Configuration::instance().app_version
    }

    pub fn session_id() -> String {
        Configuration::instance().session_id
    }

    pub fn timestamp() -> u128 {
        Configuration::instance().timestamp
    }

    pub fn find_services() -> Vec<DBServiceLite> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        config.services.iter().map(|s| DBServiceLite::new(s.1.name(), s.1.category())).collect()
    }

    pub fn find_service(key: String) -> Option<DBService> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        config.services.get(&key).cloned()
    }

    pub fn push_service(service: DBService) -> Result<DBService, ConnectException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };

        if config.services.contains_key(&service.name()) {
            let exception = ConnectException::new(String::from("Service already exists."));
            return Err(exception);
        }
        
        config.services.insert(service.name(), service.clone());
        
        return Ok(service);
    }

    pub fn put_service(service: DBService) -> DBService {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        config.services.insert(service.name(), service.clone());
        
        return service;
    }

    pub fn remove_service(service: DBService) -> Option<DBService> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => panic!("Configuration is not initialized."),
        };
        
        return config.services.remove(&service.name());
    }

}