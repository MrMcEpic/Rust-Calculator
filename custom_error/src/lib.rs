pub use std::error::Error;
pub use std::fmt;

#[derive(Debug)]
pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

impl Error for CustomError {}

impl CustomError {
    pub fn new(s: &str) -> Box<CustomError> {
        Box::new(CustomError(s.to_string()))
    }
}
