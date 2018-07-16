/*
    数値を1桁ごとに配列に格納して計算する
*/

// 桁ごとに演算した後の桁あふれを1桁上に引き継ぐ

fn normalize(x: &mut Vec<i64>, keta: i64) {
    for ii in 0..keta - 1 {
        let i = ii as usize;
        x[i + 1] += x[i] / 10;
        x[i] %= 10;
    }
}

// 足し算
fn plus(a: Vec<i64>, result: &mut Vec<i64>, keta: i64) {
    for ii in 0..keta {
        let i = ii as usize;
        result[i] += a[i];
    }
    normalize(result, keta);
}

// べき乗
fn power(x: &mut Vec<i64>, n: i64, keta: i64) {
    x[0] = 1;   // 掛算なので初期値を1にしておく
                // ｎにしないのはロジックを単純化するため
    for _ in 0..n {
        for ii in 0..keta {
            let i = ii as usize;
            x[i] *= n;
        }
        normalize(x, keta);
        x[(keta-1) as usize] = 0;
    }
}

// 表示用：関数化しておけばデバッグにも使える
fn printout(val: Vec<i64>, keta: i64) {
    for ii in (0..keta - 1).rev() {
        let i = ii as usize;
        print!("{}", val[i]);
    }
    println!("");
}

fn main() {
    const MAXKETA: i64 = 11;
    const MAXREP: i64 = 1000;
    let mut result: Vec<i64> = vec![0; MAXKETA as usize];

    for ii in 1..MAXREP + 1 {
        let i = ii as i64;
        let mut val: Vec<i64> = vec![0; MAXKETA as usize];
        power(&mut val, i, MAXKETA);
        plus(val, &mut result, MAXKETA);
    }
    printout(result, MAXKETA);
}
