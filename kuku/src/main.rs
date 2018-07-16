fn main() {
    println!("  | 1  2  3  4  5  6  7  8  9");
    println!("--+--------------------------");
    for y in 1..10 {
        print!("{:2}|", y);
        for x in 1..10 {
            print!("{:2} ", x*y);
        }
        print!("\n");
    }
}
