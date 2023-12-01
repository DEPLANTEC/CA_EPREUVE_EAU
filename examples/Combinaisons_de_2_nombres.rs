fn main() {
    for i in 0..99 {
        for j in i + 1..100 {
            println!("{:02} {:02}", i, j);
        }
    }
}
