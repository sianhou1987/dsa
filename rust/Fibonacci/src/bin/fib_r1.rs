//计算Fibonacci数列的第n项（二分递归版）：O(2^n)

use atoi::atoi;
use std::env;

fn fib_r1(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_r1(n - 1) + fib_r1(n - 2)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: {} <rank>", args[0]);
    }
    let n: u64 = atoi::<u64>(args[1].as_bytes()).unwrap();

    println!("------------- Recursion Fib 1 -------------");
    for i in 0..n + 1 {
        println!("fib({}) = {}", i, fib_r1(i));
    }
}
