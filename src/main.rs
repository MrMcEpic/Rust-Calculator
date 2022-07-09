use lib::*;

fn main() {
    let input = get_input().unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(1);
    });

    let data = get_nums(&input).unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(1);
    });

    let value = do_math(&data).unwrap_or_else(|err| {
        println!("There was a fatal error! {}", err);
        process::exit(1);
    });
    if env::var("debug").is_ok() {
        println!("input: {}", input);
        println!(
            "num 1: {}\nop: {}\nnum 2: {}",
            data.num_1, data.op, data.num_2
        );
    }
    println!("{} {} {} = {}", data.num_1, data.op, data.num_2, value);
}
