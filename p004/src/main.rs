/*
  左右どちらから読んでも同じ値になる数を回文数という.
  2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
  では, 3桁の数の積で表される回文数の最大値を求めよ.
*/

fn main() {
    println!("{}", kai());
}

fn kai() -> i32 {
    let mut rc = 0;
    for i in (100..1000).rev() {
        for j in (i..1000).rev() {
            let m = i * j;
            if rc > m {break;}
            let s = m.to_string();
            if s == s.chars().rev().collect::<String>() {
                rc = m;
                break;
            }
        }
    }
    return rc;
}
