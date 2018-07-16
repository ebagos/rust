/*
最初の10個の自然数について, その二乗の和は,
1^2 + 2^2 + ... + 10^2 = 385

最初の10個の自然数について, その和の二乗は,
(1 + 2 + ... + 10)^2 = 3025

これらの数の差は 3025 - 385 = 2640 となる.

同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.
*/
fn powsum(n: u64, sum: u64) -> u64 {
    if n == 0 {
        sum
    } else {
        println!("{}", sum);
        powsum(n - 1, sum + n * n)
    }
}

fn sumpow(n: u64, sum: u64) -> u64 {
    if n == 0 {
        sum * sum
    } else {
        sumpow(n - 1, sum + n)
    }
}

fn main() {
    let max = 100;
    let sum1 = sumpow(max, 0);
    let sum2 = powsum(max, 0);
    println!("{} => {} - {} = {}", max, sum1, sum2, sum1 - sum2);
}
