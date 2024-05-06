use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{commons::exception::connect_exception::ConnectException, domain::connection_data::ConnectionData, infrastructure::repository::{db_dictionary, i_db_repository::IDBRepository}, service::service::Service};

use super::repository::e_db_repository::EDBRepository;

#[derive(Debug, Clone)]
pub struct DBService {
    name: String,
    owner: String,
    protected: bool,
    salt: String,
    timestamp: u128,
    connection_data: ConnectionData
}

impl DBService {

    pub fn new(name: String, owner: String, protected: bool, password: String, connection_data: ConnectionData) -> Result<DBService, ConnectException> {
        let mut salt = String::new();
        if protected {
            salt = DBService::generate_salt(password)?;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cannot read actual date.")
            .as_millis();
        
        Ok(DBService {
            name, owner, protected, salt, timestamp, connection_data
        })
    }

    fn generate_salt(password: String) -> Result<String, ConnectException> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let result = argon2.hash_password(password.as_bytes(), &salt);
        if result.is_err() {
            let exception = ConnectException::new(result.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(result.unwrap().to_string())
    }

    pub fn is_authorized(&self, password: String) -> Result<(), ConnectException> {
        if !self.protected {
            return Ok(());
        }

        let parsed_hash = PasswordHash::new(&self.salt);
        if parsed_hash.is_err() {
            let exception = ConnectException::new(parsed_hash.unwrap_err().to_string());
            return Err(exception);
        }
        
        let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash.unwrap());
        if result.is_err() {
            let exception = ConnectException::new(result.unwrap_err().to_string());
            return Err(exception);
        }

        Ok(())
    }
    
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn owner(&self) -> String {
        self.owner.clone()
    }

    pub fn category(&self) -> EDBRepository {
        self.connection_data.category()
    }

    pub fn is_protected(&self) -> bool {
        self.protected
    }

    pub fn salt(&self) -> String {
        self.salt.clone()
    }

    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn connection_data(&self) -> ConnectionData {
        self.connection_data.clone()
    }

    pub fn is_same(&self, other: DBService) -> bool {
        self.name == other.name
    }

    pub async fn instance(&self) -> Result<Service<impl IDBRepository>, ConnectException> {
        let repository = db_dictionary::find(self.connection_data.clone()).await?;
        Ok(Service::from(repository))
    }

}