use std::env;

fn my_bubble_sort(array: &mut Vec<i32>) {
    let mut n = array.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if array[i - 1] > array[i] {
                array.swap(i - 1, i);
                swapped = true;
            }
        }

        n -= 1;
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

    my_bubble_sort(&mut numbers);

    for num in numbers {
        print!("{num} ");
    }
    println!();
}
