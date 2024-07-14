use std::{
    collections::HashMap, env, fs::{self, File}, io::{Read, Write}, process::Command, sync::Mutex, time::{SystemTime, UNIX_EPOCH}
};

use cargo_metadata::{CargoOpt, MetadataCommand};
use lazy_static::lazy_static;
use uuid::Uuid;

use crate::{
    commons::exception::configuration_exception::ConfigurationException,
    infrastructure::{db_service::DBService, db_service_lite::DBServiceLite},
};

const ENV_KEEP_SERVICES: &str = "KEEP_SERVICES";
const CACHE_DIRECTORY: &str = "./.cache";
const CACHE_FILE: &str = "services.json";

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
    keep_services: bool,
    services: HashMap<String, DBService>
}

impl Configuration {
    
    pub fn initialize() -> Result<Configuration, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_some() {
            //TODO: Log.
            return Err(ConfigurationException::new("Configuration is already initialized."));
        }

        let envs = Self::os_env_args();

        let rustc_version = Configuration::command_rustc_version()?;
        let cargo_version = Configuration::command_cargo_version()?;

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

        let keep_services = envs.get(ENV_KEEP_SERVICES)
            .unwrap_or(&String::new())
            .parse::<bool>()
            .unwrap_or(false);

        let services = match keep_services {
            true => Self::read_cached(),
            false => HashMap::new(),
        };

        let config = Configuration {
            rustc_version, cargo_version, app_name, app_version, session_id, timestamp, keep_services, services
        };

        *instance = Some(config);
        
        Ok(instance.as_ref().unwrap().clone())
    }

    fn command_cargo_version() -> Result<String, ConfigurationException> {
        Configuration::command_lang_version("cargo")
    }

    fn command_rustc_version() -> Result<String, ConfigurationException> {
        Configuration::command_lang_version("rustc")
    }

    fn command_lang_version(resource: &str) -> Result<String, ConfigurationException> {
        let result = Command::new(resource)
            .arg("--version")
            .output();
        if let Err(err) = result {
            return Err(ConfigurationException::new(&format!("Failed to execute command: {}", err.to_string())));
        }

        let output = result.unwrap();

        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        } else {
            //TODO: Log.
            return Err(ConfigurationException::new(&format!("Failed to get {} version", resource)));
        }
    }

    fn instance() -> Result<Configuration, ConfigurationException> {
        let instance = INSTANCE.lock().expect("Could not lock mutex");
        if instance.is_none() {
            //TODO: Log.
            return Err(ConfigurationException::new("Configuration is not initialized."))
        }
        
        Ok(instance.as_ref().unwrap().clone())
    }

    pub fn rustc_version() -> Result<String, ConfigurationException> {
        Ok(Configuration::instance()?.rustc_version)
    }

    pub fn cargo_version() -> Result<String, ConfigurationException> {
        Ok(Configuration::instance()?.cargo_version)
    }

    pub fn name() -> Result<String, ConfigurationException> {
        Ok(Configuration::instance()?.app_name)
    }

    pub fn version() -> Result<String, ConfigurationException> {
        Ok(Configuration::instance()?.app_version)
    }

    pub fn session_id() -> Result<String, ConfigurationException> {
        Ok(Configuration::instance()?.session_id)
    }

    pub fn timestamp() -> Result<u128, ConfigurationException> {
        Ok(Configuration::instance()?.timestamp)
    }

    pub fn find_services() -> Result<Vec<DBServiceLite>, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => return Err(ConfigurationException::new("Configuration is not initialized.")),
        };
        
        Ok(config.services.iter().map(|s| DBServiceLite::new(s.1.name(), s.1.category())).collect())
    }

    pub fn find_service(key: &str) -> Result<Option<DBService>, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => return Err(ConfigurationException::new("Configuration is not initialized.")),
        };
        
        Ok(config.services.get(key).cloned())
    }

    pub fn push_service(service: &DBService) -> Result<&DBService, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => return Err(ConfigurationException::new("Configuration is not initialized.")),
        };

        if config.services.contains_key(&service.name()) {
            let exception = ConfigurationException::new("Service already exists.");
            return Err(exception);
        }
        
        config.services.insert(service.name(), service.clone());
        Self::write_cached(config)?;
        
        Ok(service)
    }

    pub fn put_service(service: DBService) -> Result<Option<DBService>, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => return Err(ConfigurationException::new("Configuration is not initialized.")),
        };

        let aux = config.services.get(&service.name()).cloned();

        config.services.insert(service.name(), service.clone());
        Self::write_cached(config)?;
        
        Ok(aux)
    }

    pub fn remove_service(service: DBService) -> Result<Option<DBService>, ConfigurationException> {
        let mut instance = INSTANCE.lock().expect("Could not lock mutex");
        
        let config = match instance.as_mut() {
            Some(config) => config,
            None => return Err(ConfigurationException::new("Configuration is not initialized.")),
        };
        
        let result = config.services.remove(&service.name());
        Self::write_cached(config)?;

        Ok(result)
    }

    fn read_cached() -> HashMap<String, DBService> {
        let path = format!("{}/{}", CACHE_DIRECTORY, CACHE_FILE);
        let file = File::open(path);
        if file.is_err() {
            return HashMap::new();
        }

        let mut json = String::new();
        let result = file.unwrap().read_to_string(&mut json);
        if result.is_err() {
            return HashMap::new();
        }

        let deserialized: Result<Vec<DBService>, serde_json::Error> = serde_json::from_str(&json);
        if deserialized.is_err() {
            return HashMap::new();
        }
        
        let mut services = HashMap::new();
        for service in deserialized.unwrap() {
            services.insert(service.name(), service);
        }

        return services;
    }

    fn write_cached(configuration: &Configuration) -> Result<(), ConfigurationException> {
        if !configuration.keep_services {
            return Ok(());
        }
        
        Self::check_cache_directory()?;
        
        let path = format!("{}/{}", CACHE_DIRECTORY, CACHE_FILE);
        let file = File::create(path);
        if let Err(err) = file {
            return Err(ConfigurationException::new(&err.to_string()));
        }

        let values: Vec<&DBService> = configuration.services.values().collect();
        let serialized = serde_json::to_string_pretty(&values);
        if let Err(err) = serialized {
            return Err(ConfigurationException::new(&err.to_string()));
        }

        if let Err(err) = file.unwrap().write_all(serialized.unwrap().as_bytes()) {
            return Err(ConfigurationException::new(&err.to_string()));
        }

        return Ok(());
    }

    fn check_cache_directory() -> Result<(), ConfigurationException> {
        match fs::create_dir_all(CACHE_DIRECTORY) {
            Ok(_) => Ok(()),
            Err(err) => Err(ConfigurationException::new(&err.to_string())),
        }
    }

    pub fn os_env_args() -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (key, val) in env::vars_os() {
            if let (Ok(k), Ok(v)) = (key.into_string(), val.into_string()) {
                map.insert(k, v);
            }
        }
        return map;
    }

}