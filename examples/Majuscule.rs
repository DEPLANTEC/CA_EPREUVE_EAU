use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("error");
        return;
    }

    let input = &args[0];
    let mut result = String::new();
    let mut new_word = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            if new_word {
                result.push(c.to_uppercase().next().unwrap());
                new_word = false;
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
        } else {
            result.push(c);
            if c == ' ' || c == '\t' || c == '\n' {
                new_word = true;
            } else {
                new_word = false;
            }
        }
    }

    println!("{result}");
}
