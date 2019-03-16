pub use my_error::*;
pub use std::env;
pub use std::io;
pub use std::process;

pub struct Data {
	pub num_1: f64,
	pub op: char,
	pub num_2: f64,
}

impl Data {
	pub fn new(num_1: f64, op: char, num_2: f64) -> Data {
		Data { num_1, op, num_2 }
	}
}

pub fn get_input() -> Result<String, Box<dyn Error>> {
	println!("Input simple math");
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	input.trim();
	input = input.trim().to_string();
	Ok(input)
}

pub fn get_nums(input: &str) -> Result<Data, Box<dyn Error>> {
	let mut num_holder = String::new();
	let mut num_holder_2 = String::new();
	let mut op: Option<char> = None;
	let mut switched: bool = false;
	for c in input.chars() {
		if !c.is_digit(10) {
			switched = true;
			op = Some(c);
			continue;
		} else if !switched {
			num_holder.push(c);
		} else {
			num_holder_2.push(c);
		}
	}
	match op {
		Some(c) => match c {
			'+' | '-' | '*' | 'x' | '^' | '/' => (),
			_ => return Err(MyError::new("Invalid Operator")),
		},
		None => return Err(MyError::new("No Operator")),
	};
	let data = Data::new(
		num_holder.trim().parse()?,
		op.unwrap(),
		num_holder_2.trim().parse()?,
	);
	Ok(data)
}

pub fn do_math(data: &Data) -> Result<f64, Box<dyn Error>> {
	Ok(match data.op {
		'+' => data.num_1 + data.num_2,
		'-' => data.num_1 - data.num_2,
		'*' | 'x' => data.num_1 * data.num_2,
		'/' => data.num_1 / data.num_2,
		'^' => data.num_1.powf(data.num_2),
		_ => return Err(MyError::new("Decoding Error")),
	})
}
