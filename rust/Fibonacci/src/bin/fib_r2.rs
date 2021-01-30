//计算Fibonacci数列第n项（线性递归版）：入口形式fib(n, prev)

use atoi::atoi;
use std::env;

fn fib_r2(n: u64,  prev: &mut u64) -> u64 {
    match n {
        0 => {
            *prev = 0;
            0
        },
        1 => {
            *prev = 0;
            1
        },
        _ => {
            let mut prev_prev: u64 = 0;
            *prev = fib_r2(n-1, &mut prev_prev);
            *prev + prev_prev
        }
    }
}

// 对比迭代版的程序，可想象成当n=0时，g=0，f=1
fn fib_r2s(n: u64,  prev: &mut u64) -> u64 {
    if n == 0 {
        *prev = 1;
        0
    } else {
        let mut prev_prev: u64 = 0;
        *prev = fib_r2(n-1, &mut prev_prev);
        *prev + prev_prev
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: {} <rank>", args[0]);
    }
    let n: u64 = atoi::<u64>(args[1].as_bytes()).unwrap();

    println!("------------- Recursion Fib 2 -------------");
    let mut prev = 0;
    for i in 0..n + 1 {
        println!("fib({}) = {}", i, fib_r2(i, &mut prev));
    }

    println!("------------- Recursion Fib 2s -------------");
    let mut prev = 0;
    for i in 0..n + 1 {
        println!("fib({}) = {}", i, fib_r2(i, &mut prev));
    }
}
