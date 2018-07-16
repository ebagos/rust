#[derive(Clone)]
pub struct Item {
    pub val: i64,
    pub result: i64,
}

impl Item {
    pub fn new() -> Item {
        Item {
            val: 0,
            result: 0,
        }
    }
}

fn normalize(x: &mut Vec<Item>) -> &mut Vec<Item> {
    let len = x.len();
    for i in 0..x.len() - 1 {
        let ii = i as usize;
        x[ii + 1].val += x[ii].val / 10;
        x[ii].val %= 10;
        x[ii + 1].result += x[ii].result / 10;
        x[ii].result %= 10;
    }
    x[(len - 1) as usize].val = 0;  
    x[(len - 1) as usize].result = 0;

    return x;
}

fn plus(x: &mut Vec<Item>) {
    let len = x.len();
    for el in x {
        el.result += el.val;
    }
    let y = normalize(&mut x);
    x[(len - 1) as usize].result = 0;
}

fn power(x: &mut Vec<Item>, n: i64) {
    let len = x.len();
    x[0].val = n;
    for _ in 1..n {
        for el in x {
            el.val *= n;
        }
        normalize(&mut x);
    }
}

fn printout(x: &mut Vec<Item>) {
    for i in (0..x.len() - 1).rev() {
        let ii = i as usize;
        print!("{}", x[ii].result);
    }
    println!("");
}

fn main() {
    const MAXKETA: i64 = 11;
    const MAXREP: i64 = 1000;
    let mut numbers: Vec<Item> = vec![Item::new(); MAXKETA as usize];

    for i in 1..MAXREP + 1 {
        power(&mut numbers, i);
        plus(&mut numbers);
    }
    printout(&mut numbers);
}
