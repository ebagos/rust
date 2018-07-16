/*
正の整数に以下の式で繰り返し生成する数列を定義する.

    n → n/2 (n が偶数)

    n → 3n + 1 (n が奇数)

13からはじめるとこの数列は以下のようになる.
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

13から1まで10個の項になる. この数列はどのような数字からはじめても最終的には 1 になると考えられているが, まだそのことは証明されていない(コラッツ問題)

さて, 100万未満の数字の中でどの数字からはじめれば最長の数列を生成するか.

注意: 数列の途中で100万以上になってもよい
*/

fn collatz(n: i64) -> i64 {
    let mut count = 1;
    let mut m: i64 = n;
    while m > 1 {
        if m % 2 == 0 {
            m = m / 2;
        } else {
            m = m * 3 + 1;
        }
        count += 1;
    }
    return count;
}

fn main() {
    let mut max: i64 = 0;
    let mut key: i64 = 0;
    for i in 2..1000001 {
        let rc = collatz(i);
        if max < rc {
            max = rc;
            key = i;
        }
    }
    println!("key = {} max = {}", key, max);
    println!("910107 -> {}", collatz(910107));
}
