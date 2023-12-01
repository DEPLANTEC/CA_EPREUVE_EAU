use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("error");
        return;
    }

    let (elements, target) = args.split_at(args.len() - 1);
    let target = &target[0];

    let index = elements.iter().position(|e| e == target);

    match index {
        Some(i) => println!("{i}"),
        None => println!("-1"),
    }
}
