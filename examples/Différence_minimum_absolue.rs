use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("error");
        return;
    }

    let mut numbers: Vec<i32> = Vec::new();
    for arg in args {
        match arg.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                eprintln!("error");
                return;
            }
        }
    }

    numbers.sort_unstable();

    let mut min_diff = i32::MAX;
    for window in numbers.windows(2) {
        let diff = (window[1] - window[0]).abs();
        if diff < min_diff {
            min_diff = diff;
        }
    }

    println!("{min_diff}");
}
