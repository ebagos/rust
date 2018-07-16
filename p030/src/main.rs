/*
驚くべきことに, 各桁を4乗した数の和が元の数と一致する数は3つしかない.

    1634 = 1**4 + 6**4 + 3**4 + 4**4
    8208 = 8**4 + 2**4 + 0**4 + 8**4
    9474 = 9**4 + 4**4 + 7**4 + 4**4

ただし, 1=1**4は含まないものとする. この数たちの和は 1634 + 8208 + 9474 = 19316 である.

各桁を5乗した数の和が元の数と一致するような数の総和を求めよ.
*/
/*
なお、本プログラムには問題が内包されている。
その問題を指摘し修正せよ。
*/
fn powi(x: u32, n: u32, base: u32) -> u32 {
    if n == 0 {
        base
    } else {
        powi(x, n - 1, base * x)
    }
}

fn fill(n: u32, c: u32, sum: u32) -> u32 {
    if c == 0 {
        sum
    } else {
        fill(n, c - 1, sum * 10 + n)
    }
}

fn limit(n: u32, keta: u32) ->u32 {
    if fill(9, keta, 0) > keta * powi(9, n, 1) {
        fill(9, keta, 0)
    } else {
        limit(n, keta + 1)
    }
}

fn calc(x: u32, n: u32, base: u32) -> u32 {
    if x < 10 {
        base + powi(x, n, 1)
    } else {
        calc(x / 10, n, powi(x % 10, n, 1) + base)
    }
}

fn problem_30(n: u32, sum: u32) -> u32 {
    if n > limit(5, 1) {
        sum
    } else {
        if n == calc(n, 5, 0) {
            problem_30(n + 1, sum + n)
        } else {
            problem_30(n + 1, sum)
        }
    }
}

fn main() {
    for i in 2..limit(5, 1) {
        if i == calc(i, 5, 0) {
            println!("{}", i)
        }
    }
    println!("sum = {}", problem_30(2, 0));
}
