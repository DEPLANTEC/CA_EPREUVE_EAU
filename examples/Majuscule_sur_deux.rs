use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("error");
        return;
    }

    let input = &args[0];
    let mut toggle = false;
    let mut result = String::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            result.push(if toggle { c.to_uppercase().next().unwrap() } else { c });
            toggle = !toggle;
        } else {
            result.push(c);
        }
    }

    println!("{result}");
}
