fn main() {
    let max = 40;

    println!("再帰");
    for x in 0..max+1 {
        println!("再帰　　 {} : {}", x, fibo(x));
    }
    println!("\n末尾再帰");
    for x in 0..max+1 {
        println!("末尾再帰 {} : {}", x, fiboi(x, 1, 0));
    }
}

// 再帰
fn fibo(n: u32) -> u32 {
    match n {
        0 => return 1,
        1 => return 1,
        _ => return fibo(n - 2) + fibo(n - 1),
    }
}

// 末尾再帰
fn fiboi(n: u32, a: u32, b: u32) -> u32 {
    return if n == 0 {a} else {fiboi(n-1, a+b, a)}
}