/*
5000個以上の名前が書かれている46Kのテキストファイル names.txt を用いる. まずアルファベット順にソートせよ.

のち, 各名前についてアルファベットに値を割り振り, リスト中の出現順の数と掛け合わせることで, 名前のスコアを計算する.

たとえば, リストがアルファベット順にソートされているとすると, COLINはリストの938番目にある. またCOLINは 3 + 15 + 12 + 9 + 14 = 53 という値を持つ. よってCOLINは 938 × 53 = 49714 というスコアを持つ.

ファイル中の全名前のスコアの合計を求めよ.
*/

// Stringを多用しているかも＆参照とかコピーとかぐちゃぐちゃかも
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_sub(path: &Path) -> io::Result<String> {
    let mut f = File::open(path).unwrap();
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}

fn split(src: String) -> Vec<String> {
    let mut dst: Vec<String> = vec![];
    for s in src.split(",") {
        let ss = s.replace("\"", "");
        dst.push(ss.to_string());
    }
    return dst;
}

fn calc(x: i32, s: String) -> i32 {
    let mut rc = 0;
    let bytes = s.into_bytes();
    for n in 0..bytes.len() {
        rc += bytes[n] as i32 - 'A' as i32 + 1;
    }
    return x * rc;
}

fn main() {
    let mut dst: Vec<String> = vec![];
    match read_sub(&Path::new("names.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => dst = split(s),
    }
    let mut sum: i32 = 0;
    dst.sort();
    for i in 0..dst.len() {
        let s = dst[i].to_string();
        sum += calc(1 + i as i32, s);
    }
    println!("sum = {}", sum);
}
