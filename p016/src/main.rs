/*
2^15 = 32768 であり, 各位の数字の和は 3 + 2 + 7 + 6 + 8 = 26 となる.

同様にして, 2^1000 の各位の数字の和を求めよ.
*/

// 言語仕様でない限り変数の最大ビット長で破綻するので工夫が必要
fn main() {
    const MAX: usize  = 500;
    let mut num = vec![0; MAX];
    num[0] = 1;
    for _n in 1..1001 {
        num = num.iter().map(|&x| x * 2).collect::<Vec<_>>();
/*
        for m in 0..MAX {
            num[m] *= 2;
        }
*/
        for m in 0..MAX-1 {
            let x = num[m] / 10;
            num[m+1] += x;
            num[m] -= x * 10;
        }
    }
    /*
    for m in (0..MAX).rev() {
        if num[m] != 0 {
            print!("{}", num[m]);
        }
    }
    */
    num.iter().rev().for_each(|x| print!("{}", x));
    println!("");
 
    let ans: i64 = num.iter().sum();
    println!("answer = {}", ans)
}
