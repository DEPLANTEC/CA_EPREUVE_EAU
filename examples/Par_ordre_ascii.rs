use std::env;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("error");
        return;
    }

    args.sort_unstable();

    for arg in args {
        print!("{arg} ");
    }
    println!();
}
