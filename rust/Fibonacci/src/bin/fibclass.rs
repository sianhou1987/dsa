use atoi::atoi;
use fibonacci::Fib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: {} <rank>", args[0]);
    }
    let n: u64 = atoi::<u64>(args[1].as_bytes()).unwrap();

    println!("------------- class Fib -------------");
    let mut f = Fib::new(0);
    for i in 0..n + 1 {
        println!("fib({}) = {}", i, f.get());
        f.next();
    }
}
