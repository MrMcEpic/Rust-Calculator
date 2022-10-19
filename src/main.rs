use lib::*;
pub use std::env;

fn env_get() -> (Option<String>, bool) {
    let mut args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    args.remove(0);
    let mut input: String = String::new();
    for i in args {
        if i != "debug" {
            input += &i;
        } else {
            debug = true;
        }
    }
    if input.trim().is_empty() {
        return (None, debug);
    }
    (Some(input), debug)
}

fn main() {
    let (mut input, debug) = env_get();
    if let None = input {
        input = Some(get_input().unwrap_or_else(|err| CustomError::state_error(err)));
    }
    let input: String = input.unwrap();
    let mut data = Data::initalize(&input).unwrap_or_else(|err| CustomError::state_error(err));

    data.do_math()
        .unwrap_or_else(|err| CustomError::state_error(err));

    if debug {
        println!(
            "input: {}\nnum 1: {}\nop: {}\nnum 2: {:?}",
            input, data.num_1, data.operator, data.num_2
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
