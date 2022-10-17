use lib::*;
pub use std::env;

fn env_get() -> (String, bool) {
    let mut args: Vec<String> = env::args().collect();
    let mut debug: bool = false;
    args.remove(0);
    let mut input: String = String::new();
    for i in args {
        if i != "debug" {
            input.push_str(&i);
        } else {
            debug = true;
        }
    }
    (input, debug)
}

fn main() {
    let (input, debug) = env_get();
    let mut data = Data::initalize(&input).unwrap_or_else(|err| CustomError::state_error(err));

    data.do_math()
        .unwrap_or_else(|err| CustomError::state_error(err));

    if debug {
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
