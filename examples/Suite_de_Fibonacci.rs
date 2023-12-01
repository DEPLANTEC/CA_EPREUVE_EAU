use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("-1");
        return;
    }

    let n = match args[0].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("-1");
            return;
        }
    };

    println!("{}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut sum;

    if n == 0 {
        return a;
    }

    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}
