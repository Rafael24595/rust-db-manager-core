use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ConnectException {
    message: String,
}

impl fmt::Display for ConnectException {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConnectException: {}", self.message)
    }
    
}

impl Error for ConnectException {}

impl ConnectException {
    
    pub fn new(message: String) -> ConnectException {
        ConnectException {
            message
        }
    }

    pub fn new_str(message: &str) -> ConnectException {
        Self::new(String::from(message))
    }
    
    pub fn message(&self) -> &str {
        &self.message
    }

}