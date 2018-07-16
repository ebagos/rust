/*
13195 の素因数は 5, 7, 13, 29 である
600851475143 の素因数のうち最大のものを求めよ
*/
fn factor(n: u64) -> u64 {
    let mut x = 2;
    let mut nn = n;
    while x * x <= nn {
        while nn % x == 0 {
            nn /= x;
        }
        x += 1;
    }
    if nn == 1 {
        x
    } else {
        nn
    }
}
fn factor2_sub(n: u64, x: u64) -> u64 {
    if x * x > n  {
        if n == 1 {
            x
        } else {
            n
        }
    } else {
        if n % x == 0 {
            factor2_sub(n / x, x)
        } else {
            factor2_sub(n, x + 1)
        }
    }
}
fn factor2(n: u64) -> u64 {
    return factor2_sub(n, 2);
}

fn main() {
    println!("600851475143の最大の素因数 = {}", factor(600851475143));
    println!("600851475143の最大の素因数 = {}", factor2(600851475143));
}
