use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("error");
        std::process::exit(1);
    }

    for arg in args.iter().rev() {
        println!("{arg}");
    }
}
