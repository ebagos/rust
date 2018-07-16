/*
2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.
*/

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn sub(a: u64, b: u64, c: u64) -> u64 {
    if b == c {
        a
    } else {
        sub(lcm(a, b), b + 1, c)
    }
}

fn main() {
    println!("{}", sub(1, 1, 20));
}
