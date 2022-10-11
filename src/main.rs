use lib::*;
pub use std::env;
pub use std::process;

fn main() {
    let input = get_input().unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(0);
    });

    let data = Data::get_nums(&input).unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(0);
    });

    let value = data.do_math().unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(0);
    });
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
            value
        );
    } else {
        println!("{}{} = {}", data.operator, data.num_1, value);
    }
}
