//计算Fibonacci数列的第n项（迭代版）：O(n)

use atoi::atoi;
use std::env;

fn fib_i(mut n: u64) -> u64 {
    let mut g: u64 = 0;
    let mut f: u64 = 1;
    while n > 1 {
        g = g + f;
        f = g - f;
        n = n - 1;
    }
    g
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: {} <rank>", args[0]);
    }
    let n: u64 = atoi::<u64>(args[1].as_bytes()).unwrap();

    println!("------------- Iteration Fib -------------");
    for i in 0..n + 1 {
        println!("fib({}) = {}", i, fib_i(i));
    }
}
