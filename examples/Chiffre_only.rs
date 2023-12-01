use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("error");
        return;
    }

    let input = &args[0];
    let is_numeric = input.chars().all(char::is_numeric);

    println!("{}", is_numeric);
}
