/*
各桁の2乗を足し合わせて新たな数を作ることを, 同じ数が現れるまで繰り返す.

例えば

　　44 → 32 → 13 → 10 → 1 → 1
　　85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89

のような列である. どちらも1か89で無限ループに陥っている.
驚くことに, どの数から始めても最終的に1か89に到達する.

では, 10,000,000より小さい数で89に到達する数はいくつあるか.
*/
use std::time::Instant;

fn square_plus(n: i32, s: i32) -> i32 {
    if n == 0 {
        return s;
    }
    let x = n % 10;
    return square_plus(n / 10, s + x * x);
}

fn main() {
    let start = Instant::now();
    let mut count = 0;
    let MAX: usize = (9i32.pow(2) * 7 + 1) as usize;
    let mut memo = vec![-1; MAX];
    memo[1] = 0;
    memo[89] = 1;

    for i in 2..10_000_000 {
        let mut n = i;
        loop {
            n = square_plus(n, 0);
            if memo[n as usize] != -1 {
                break;
            }
        }
        if memo[n as usize] == 1 {
            count += 1;
            if i < MAX as i32 {
                memo[i as usize] = 1;
            }
        } else if i < MAX as i32 {
            memo[i as usize] = 0;
        }
    }
    let end = start.elapsed();
    println!("count = {}", count);
    println!("elapsed : {}.{} sec", end.as_secs(), end.subsec_nanos());
}
