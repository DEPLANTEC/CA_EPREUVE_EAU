use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("-1");
        return;
    }

    let num = match args[0].parse::<i64>() {
        Ok(n) if n >= 0 => n,
        _ => {
            eprintln!("-1");
            return;
        }
    };

    println!("{}", find_next_prime(num));
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn find_next_prime(mut n: i64) -> i64 {
    loop {
        n += 1;
        if is_prime(n) {
            return n;
        }
    }
}
