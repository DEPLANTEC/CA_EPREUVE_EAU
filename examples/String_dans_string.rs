use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("error");
        return;
    }
    let (haystack, needle) = (&args[0], &args[1]);
    println!("{}", haystack.contains(needle));
}
