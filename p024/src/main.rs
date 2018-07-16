/*
順列とはモノの順番付きの並びのことである. たとえば, 3124は数 1, 2, 3, 4 の一つの順列である. すべての順列を数の大小でまたは辞書式に並べたものを辞書順と呼ぶ. 0と1と2の順列を辞書順に並べると
012 021 102 120 201 210
になる.

0,1,2,3,4,5,6,7,8,9からなる順列を辞書式に並べたときの100万番目はいくつか?
*/

fn fact(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    return n * fact(n - 1);
}

fn miso(n: u32, x: u32, left: u32, v: &mut Vec<u32>, ar: &mut Vec<u32>) {
    let k = fact(n - 1);
    let idx = x / k;
    let am = x % k;

    ar[left as usize] = v[idx as usize];
    v.remove(idx as usize);
    if n == 1 {
        return;
    }
    return miso(n - 1, am, left + 1, v, ar);
}

fn main() {
    let keta = 10;
    let place = 1_000_000;
  
    let mut v: Vec<u32> = vec![];
    let mut ar: Vec<u32> = vec![];
    for i in 0..keta {
        v.push(i);
        ar.push(0);
    }
    println!("size of v = {}", v.len());
    miso(keta, place - 1, 0, &mut v, &mut ar);
    println!("size of v = {}", v.len());
    for i in 0..keta {
        print!("{}", ar[i as usize]);
    }
    println!("");
}
