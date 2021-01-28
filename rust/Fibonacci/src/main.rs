use atoi::atoi;
use std::env;
use Fibonacci::FibClass;

fn main() {
    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: {} <rank>\n", args[0]);
    }
    let n: u64 = atoi::<u64>(args[1].as_bytes()).unwrap();

    println!("\n------------- class Fib -------------\n");
    let mut f = FibClass::new(0);
    for i in 0..n {
        println!("fib({}) = {}", i, f.get());
        f.next();
    }
}
