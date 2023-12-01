fn main() {
    for i in 0..=9 {
        for j in 0..=9 {
            if j == i {
                continue;
            }
            for k in 0..=9 {
                if k == i || k == j {
                    continue;
                }
                println!("{i}{j}{k}");
            }
        }
    }
}
