use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ConfigurationException {
    message: String,
}

impl fmt::Display for ConfigurationException {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigurationException: {}", self.message)
    }
    
}

impl Error for ConfigurationException {}

impl ConfigurationException {
    
    pub fn new(message: &str) -> ConfigurationException {
        return ConfigurationException {
            message: message.to_string()
        };
    }
    
    pub fn message(&self) -> String {
        return self.message.clone();
    }

}