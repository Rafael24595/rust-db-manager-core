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
        return ConnectException {
            message
        };
    }
    
    pub fn message(&self) -> String {
        return self.message.clone();
    }

}