use std::{
    collections::HashMap, env, fs::{self, File}, io::{Read, Write}, process::Command, sync::Arc, time::{SystemTime, UNIX_EPOCH}
};

use cargo_metadata::{CargoOpt, MetadataCommand};
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    commons::exception::configuration_exception::ConfigurationException,
    infrastructure::{db_service::DBService, db_service_lite::DBServiceLite},
};

use super::db_connection::DBConnection;

const ENV_KEEP_SERVICES: &str = "KEEP_SERVICES";
const CACHE_DIRECTORY: &str = "./.cache";
const CACHE_FILE: &str = "services.json";

lazy_static! {
    static ref INSTANCE: Arc<RwLock<Configuration>> = Arc::new(RwLock::new(Configuration::empty()));
}

pub struct Configuration {
    rustc_version: String,
    cargo_version: String,
    app_name: String,
    app_version: String,
    session_id: String,
    timestamp: u128,
    keep_services: bool,
    services: HashMap<String, RwLock<DBConnection>>
}

impl Configuration {
    
    fn empty() -> Configuration {
        Configuration { 
            rustc_version: String::new(), 
            cargo_version: String::new(), 
            app_name: String::new(), 
            app_version: String::new(), 
            session_id: String::new(),
            timestamp: 0, 
            keep_services: false, 
            services: HashMap::new()
        }
    }

    pub async fn initialize() -> Result<Arc<RwLock<Configuration>>, ConfigurationException> {
        let instance = &INSTANCE;
        let mut instance = instance.write().await;
        if instance.is_initialized() {
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
        
        *instance = Configuration {
            rustc_version, cargo_version, app_name, app_version, session_id, timestamp, keep_services, services
        };
        
        Ok(Arc::clone(&INSTANCE))
    }

    pub async fn instance() -> Result<Arc<RwLock<Configuration>>, ConfigurationException> {
        let instance = Arc::clone(&INSTANCE);
        if instance.read().await.is_not_initialized() {
            //TODO: Log.
            return Err(ConfigurationException::new("Configuration is not initialized."))
        }

        Ok(Arc::clone(&INSTANCE))
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

    fn is_initialized(&self) -> bool {
        !self.cargo_version.is_empty() && !self.rustc_version.is_empty()
    }

    fn is_not_initialized(&self) -> bool {
        !self.is_initialized()
    }

    pub fn rustc_version(&self) -> &str {
        &self.rustc_version
    }

    pub fn cargo_version(&self) -> &str {
        &self.cargo_version
    }

    pub fn name(&self) -> &str {
        &self.app_name
    }

    pub fn version(&self) -> &str {
        &self.app_version
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }

    pub async fn find_services(&self) -> Vec<DBServiceLite> {
        let mut services = Vec::new();
        for (_, service) in &self.services {
            let binding = service.read().await;
            let config = binding.configuration();
            services.push(DBServiceLite::new(config.name().to_string(), config.is_protected(), config.category().clone()));
        }
        services
    }

    pub fn find_service(&self, key: &str) -> Option<&RwLock<DBConnection>> {
        self.services.get(key)
    }

    pub async fn push_service(&mut self, service: DBService) -> Result<DBService, ConfigurationException> {
        if self.services.contains_key(service.name()) {
            let exception = ConfigurationException::new("Service already exists.");
            return Err(exception);
        }
        
        let connection = RwLock::new(DBConnection::new(&service));

        self.services.insert(service.name().to_string(), connection);
        self.write_cached().await?;
        
        Ok(service)
    }

    pub async fn put_service(&mut self, service: DBService) -> Result<Option<DBService>, ConfigurationException> {
        let config = Self::instance().await?;
        let config = config.write().await;

        let aux = self.services.get(service.name());

        let mut schema = None;
        if let Some(aux) = aux {
            let conn = aux.read().await;
            schema = Some(conn.configuration().clone());
        }

        let connection = RwLock::new(DBConnection::new(&service));

        self.services.insert(service.name().to_string(), connection);
        Self::write_cached(&config).await?;

        Ok(schema)
    }

    pub async fn remove_service(&mut self, service: DBService) -> Result<Option<DBService>, ConfigurationException> {
        let result = self.services.remove(service.name());

        let mut schema = None;
        if let Some(aux) = result {
            let conn = aux.read().await;
            schema = Some(conn.configuration().clone());
        }
        
        self.write_cached().await?;

        Ok(schema)
    }

    fn read_cached() -> HashMap<String, RwLock<DBConnection>> {
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
            let connection = RwLock::new(DBConnection::new(&service));
            services.insert(service.name().to_string(), connection);
        }

        services
    }

    async fn write_cached(&self) -> Result<(), ConfigurationException> {
        if !self.keep_services {
            return Ok(());
        }
        
        Self::check_cache_directory()?;
        
        let path = format!("{}/{}", CACHE_DIRECTORY, CACHE_FILE);
        let file = File::create(path);
        if let Err(err) = file {
            return Err(ConfigurationException::new(&err.to_string()));
        }

        let mut services = Vec::new();
        for service in self.services.values() {
            let conn = service.read().await;
            let schema = conn.configuration();
            services.push(schema.clone());
        }            

        let serialized = serde_json::to_string_pretty(&services);
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