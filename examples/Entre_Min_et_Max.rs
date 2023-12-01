use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        eprintln!("error");
        return;
    }

    let start = match args[0].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("error");
            return;
        },
    };

    let end = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("error");
            return;
        },
    };

    let (min, max) = if start < end { (start, end) } else { (end, start) };

    for number in min..max {
        print!("{number} ");
    }
    println!();
}
