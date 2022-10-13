pub use std::error::Error;
pub use std::fmt;
pub use std::process;

#[derive(Debug)]
pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for CustomError {}

impl CustomError {
    pub fn new(s: &str) -> Box<CustomError> {
        Box::new(CustomError(s.to_string()))
    }

    pub fn state_error(err: Box<dyn Error>) -> ! {
        println!("There was a fatal error! {}", err);
        process::exit(0);
    }
}
