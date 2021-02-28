use std::vec;

struct Solution;

impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        Solution::hanota_impl(a, b, c, a.len());
    }

    fn hanota_impl(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>, n: usize) {
        if n > 0 {
            Solution::hanota_impl(a, c, b, n - 1);
            c.push(a.pop().unwrap());
            Solution::hanota_impl(b, a, c, n - 1);
        }
    }
}

fn main() {
    let mut a: Vec<i32> = vec![2, 1, 0];
    let mut b: Vec<i32> = Vec::with_capacity(a.len());
    let mut c: Vec<i32> = Vec::with_capacity(a.len());

    println!("Before: a = {:?}; b = {:?}; c = {:?}.", a, b, c);
    Solution::hanota(&mut a, &mut b, &mut c);
    println!("After: a = {:?}; b = {:?}; c = {:?}.", a, b, c);
}
