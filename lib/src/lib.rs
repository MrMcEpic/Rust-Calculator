pub use custom_error::*;
pub use std::io;

pub struct Data {
    pub num_1: f64,
    pub operator: char,
    pub num_2: Option<f64>,
    pub value: Option<f64>,
}

// macro_rules! _get_nums {
//     ($($a:expr, $b:expr, $c:expr), *) => {
// 		let mut list = Vec::new();
// 		$(
// 			let num_1 = $a;
// 			let operator = $b;
// 			let num_2 = $c;
// 			list.push(Data::new(num_1, operator, num_2));
// 		)*
// 		list
// 	};
// }

impl Data {
    ///Handles the math logic and returns a [`f64`] wrapped in a [`Result`].
    pub fn do_math(&mut self) -> Result<f64, Box<dyn Error>> {
        let value = if let Some(num_2) = self.num_2 {
            match self.operator {
                '+' => self.num_1 + &num_2,
                '-' => self.num_1 - num_2,
                '*' | 'x' => self.num_1 * num_2,
                '/' => self.num_1 / num_2,
                '^' => self.num_1.powf(num_2),
                '%' => self.num_1 % num_2,
                _ => return Err(CustomError::new("Decoding Error")),
            }
        } else {
            if self.num_1 < 0.0 || self.num_1 % 1.0 != 0.0 {
                return Err(CustomError::new("Invalid Number"));
            }
            match self.operator {
                '!' => self.factorial(self.num_1),
                'f' => self.fibonacci(self.num_1),
                _ => return Err(CustomError::new("Decoding Error")),
            }
        };
        self.value = Some(value);
        Ok(value)
    }

    fn factorial(&self, x: f64) -> f64 {
        //Only works for positive whole numbers for now.
        if x == 0.0 {
            1.0
        } else {
            x * self.factorial(x - 1.0)
        }
    }

    fn fibonacci(&self, x: f64) -> f64 {
        //Only works for positive whole numbers for now.
        if x == 0.0 {
            0.0
        } else if x == 1.0 {
            1.0
        } else {
            self.fibonacci(x - 1.0) + self.fibonacci(x - 2.0)
        }
    }

    fn get_nums(input: &str) -> Result<(f64, char, Option<f64>), Box<dyn Error>> {
        let mut num_holder: String = String::new();
        let mut num_holder_2: String = String::new();
        let mut num_holder_2_to_send: Option<f64> = None;
        let mut operator: Option<char> = None;
        let mut switched: bool = false;
        let mut operator_checked: bool = false;
        let mut active_quotes: bool = false;
        for c in input.chars() {
            if c == '"' || c == '\'' {
                //One way to do negative numbers
                active_quotes = !active_quotes;
            } else if c.is_digit(10) || c == '.' || (active_quotes && c == '-') {
                if !switched {
                    num_holder.push(c);
                } else {
                    num_holder_2.push(c);
                }
            } else if c == 'n' {
                //This is how we do negative numbers for now
                if !switched {
                    num_holder.push('-');
                } else {
                    num_holder_2.push('-');
                }
            } else if !operator_checked
                && (c == '+'
                    || c == '-'
                    || c == '*'
                    || c == 'x'
                    || c == '/'
                    || c == '^'
                    || c == '%'
                    || c == '!'
                    || c == 'f')
            {
                //TODO Make a static array/tuple containting all of the valid operators
                if c != '!' && c != 'f' {
                    switched = true;
                }
                operator_checked = true;
                operator = Some(c);
            }
        }

        let operator: char = if let Some(c) = operator {
            if let '+' | '-' | '*' | 'x' | '^' | '/' | '%' | '!' | 'f' = c {
                if c != '!' && c != 'f' {
                    num_holder_2_to_send = Some(num_holder_2.trim().parse()?);
                }
                c
            } else {
                return Err(CustomError::new("Invalid Operator"));
            }
        } else {
            return Err(CustomError::new("No Operator"));
        };

        Ok((num_holder.trim().parse()?, operator, num_holder_2_to_send))
    }

    ///Takes a user input [`str`] and returns a [`Data`] struct wrapped in a [`Result`].
    pub fn initalize(input: &str) -> Result<Data, Box<dyn Error>> {
        Ok(Data::new(Data::get_nums(input)?))
    }

    ///Takes a [`f64`], [`char`], and [`Option<f64>`] and returns a [`Data`] struct.
    fn new((num_1, operator, num_2): (f64, char, Option<f64>)) -> Data {
        Data {
            num_1,
            operator,
            num_2,
            value: None,
        }
    }
}

///Gets user input from the terminal and returns it as a [`String`] wrapped in a [`Result`].
pub fn get_input() -> Result<String, Box<dyn Error>> {
    println!("Input simple math");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_lowercase().to_string();
    Ok(input)
}

// fn _seperator(input: &str) {
//     //Maybe make an enum that has a tuple option that holds the numbers so that there can be a proper list of the numbers
//     let mut counter = 0;
//     let mut holder = String::new();
//     let mut list: Vec<String> = Vec::new();
//     for i in input.chars() {
//         if let '+' | '-' | '*' | 'x' | '^' | '/' | '%' | '!' | 'f' = i {
//             counter += 1;
//         }
//         if counter % 2 == 0 {
//             list.push(holder);
//             holder = String::new();
//             counter = 0;
//         } else {
//             holder.push(i);
//         }
//     }
// }
