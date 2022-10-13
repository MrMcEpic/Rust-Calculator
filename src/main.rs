use lib::*;
pub use std::env;

fn main() {
    let input = get_input().unwrap_or_else(|err| CustomError::state_error(err));

    let mut data = Data::initalize(&input).unwrap_or_else(|err| CustomError::state_error(err));

    data.do_math()
        .unwrap_or_else(|err| CustomError::state_error(err));

    if env::var("debug").is_ok() {
        println!("input: {}", input);
        println!(
            "num 1: {}\nop: {}\nnum 2: {:?}",
            data.num_1, data.operator, data.num_2
        );
    }

    if data.num_2.is_some() {
        println!(
            "{} {} {} = {}",
            data.num_1,
            data.operator,
            data.num_2.unwrap(),
            data.value.unwrap()
        );
    } else {
        println!("{}{} = {}", data.operator, data.num_1, data.value.unwrap());
    }
}
