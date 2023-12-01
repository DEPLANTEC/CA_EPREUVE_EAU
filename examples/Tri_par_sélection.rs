use std::env;

fn my_select_sort(array: &mut Vec<i32>) {
    let n = array.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if array[j] < array[min_idx] {
                min_idx = j;
            }
        }
        array.swap(i, min_idx);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("error");
        return;
    }

    let mut numbers: Vec<i32> = match args.iter().map(|arg| arg.parse()).collect() {
        Ok(nums) => nums,
        Err(_) => {
            eprintln!("error");
            return;
        },
    };

    my_select_sort(&mut numbers);

    for num in &numbers {
        print!("{num} ");
    }
    println!();
}
