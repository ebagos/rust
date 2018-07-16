/*
    数値を1桁ごとに配列に格納して計算する
*/
use std::ops::Add;
use std::ops::Mul;

struct Valarray {
    val: Vec<i64>,
    length: i64,
}

impl Valarray {
    pub fn new(length: i64) -> Valarray {
        Valarray {
            length: length,
            val: vec![0; length as usize],
        }
    }
}

impl Add for Valarray {
    type Output = Valarray;

    fn add(self, other: Valarray) -> Valarray {
        let mut result = Valarray::new(self.length);
        for i in 0..self.length - 1 {
            let ii = i as usize;
            result.val[ii] = self.val[ii] + other.val[ii];
        }
//        normalize(&mut result.val, result.length);
        normalize(&mut result);
        result.val[(result.length - 1) as usize] = 0;
        return result;
    }
}

impl Mul<i64> for Valarray {
    type Output = Valarray;

    fn mul(self, rhs: i64) -> Valarray {
        let mut result = Valarray::new(self.length);
        for i in 0..self.length - 1 {
            let ii = i as usize;
            result.val[ii] = self.val[ii] * rhs;
        }
//        normalize(&mut result.val, result.length);
        normalize(&mut result);
        result.val[(result.length - 1) as usize] = 0;
        return result;
    }
}

// 桁ごとに演算した後の桁あふれを1桁上に引き継ぐ
/*
fn normalize(x: &mut Vec<i64>, keta: i64) {
    for ii in 0..keta - 1 {
        let i = ii as usize;
        x[i + 1] += x[i] / 10;
        x[i] %= 10;
    }
}
*/

fn normalize(x: &mut Valarray) {
    for i in 0..x.length - 1 {
        let ii = i as usize;
        x.val[ii + 1] += x.val[ii] / 10;
        x.val[ii] %= 10;
    }
}

// べき乗
fn power(x: Valarray, n: i64) -> Valarray {
    let mut result = Valarray::new(x.length);
    for i in 0..x.length {
        let ii = i as usize;
        result.val[ii] = x.val[ii];
    }
    for _ in 1..n {
        result = result * n;
    }
    return result;
}

// 表示用：関数化しておけばデバッグにも使える
fn printout(x: &mut Valarray) {
    for ii in (0..x.length - 1).rev() {
        let i = ii as usize;
        print!("{}", x.val[i]);
    }
    println!("");
}

fn main() {
    const MAXKETA: i64 = 11;
    const MAXREP: i64 = 1000;
    let mut result = Valarray::new(MAXKETA);

    for i in 1..MAXREP + 1 {
        let mut val = Valarray::new(MAXKETA);
        val.val[0] = i;
        normalize(&mut val);
        result = result + power(val, i);
    }
    printout(&mut result);
}
