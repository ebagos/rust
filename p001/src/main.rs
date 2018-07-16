/*
  1以上、1000未満の整数で、3または5で割り切れるものの和を求める
  複数の解決法を提示し、評価せよ
*/

fn main() {
    // 一般的なC/C++のようなコーディング
    let mut sum: i64 = 0;
    for i in 1..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }
    println!("sum = {}", sum);

    // 再帰
    println!("sum = {}", p001_sub(999, 0));

    // レンジの機能
    let s:i64 = (1..1000).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum();
    println!("sum = {}", s);

    // 直接マクロに渡すとエラー（この理由を知るべし）
//    println!("sum = {}", s = (1..1000).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum());
    // だからといって関数化するほどのものでもない
    println!("sum = {}", p001_sub_2(1,1000));
}

fn p001_sub(n: i64, sum: i64) -> i64 {
    match n {
        1 => return sum,
        _ => if (n % 3 == 0) || (n % 5 == 0) {
                return p001_sub(n - 1, sum + n);
            } else {
                return p001_sub(n - 1, sum);
            }
    };
}

fn p001_sub_2(s: i64, e:i64) -> i64 {
    (s..e).filter(|x| (x % 3 == 0) || (x % 5 == 0)).sum()
}
