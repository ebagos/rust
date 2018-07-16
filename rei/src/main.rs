fn sum1(n: i64) ->i64 {
    let mut sum: i64 = 0;
    for i in 1..n+1 {
        sum += i;
    }
    return sum;
}

fn sum2(n: i64) -> i64 {
    if (n < 0) {
        return 0;
    }
    return (1+n) * n / 2;
}

fn sum3(n: i64) -> i64 {
    (1..n+1).sum()
}

fn main() {
    for i in 65530..i64::max_value() {
        println!("ans1 = {}", sum1(i));
        println!("ans2 = {}", sum2(i));
        println!("ans3 = {}", sum3(i));
    }
    println!("Hello, world!");
}
