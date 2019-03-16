pub use std::error::Error;
pub use std::fmt;

#[derive(Debug)]
pub struct MyError(String);

impl fmt::Display for MyError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "error: {}", self.0)
	}
}

impl Error for MyError {}

impl MyError {
	pub fn new(s: &str) -> Box<MyError> {
		Box::new(MyError(s.to_string()))
	}
}