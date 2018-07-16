/*
n × (n - 1) × ... × 3 × 2 × 1 を n! と表す.

例えば, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800 となる.
この数の各桁の合計は 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27 である.

では, 100! の各桁の数字の和を求めよ.
*/
fn factorial(n: u64, ans: u64) -> u64 {
    if n == 0 {
        ans
    } else {
        factorial(n - 1, ans * n)
    }
}

fn nsum(n: u64, sum: u64) -> u64 {
    if n == 0 {
        sum
    } else {
        nsum(n / 10, sum + n % 10)
    }
}
fn main() {
    let max1 = 10;
    let max2 = 100;

    println!("{} => {}", max1, factorial(max1, 1));
    println!("{} => {}", max1, nsum(factorial(max1, 1), 0));

    println!("{} => {}", max2, factorial(max2, 1));
    println!("{} => {}", max2, nsum(factorial(max2, 1), 0));
}
