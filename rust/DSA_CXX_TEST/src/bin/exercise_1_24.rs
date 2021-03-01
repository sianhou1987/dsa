struct Solution;

impl Solution {
    fn cover(
        n: usize,
        mid_x: usize,
        mid_y: usize,
        x: usize,
        y: usize,
        l: &mut i32,
        dp: &mut Vec<Vec<i32>>,
    ) {
        let s = n >> 1;

        if x >= mid_x && y >= mid_y {
            dp[mid_y - 1][mid_x - 1] = *l;
            dp[mid_y - 1][mid_x] = *l;
            dp[mid_y][mid_x - 1] = *l;
            // dp[mid_y][mid_x] = l;
            if n >= 4 {
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y + s / 2, x, y, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y - s / 2, mid_x, mid_y - 1, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x - s / 2, mid_y + s / 2, mid_x - 1, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(
                    n / 2,
                    mid_x - s / 2,
                    mid_y - s / 2,
                    mid_x - 1,
                    mid_y - 1,
                    l,
                    dp,
                );
            }
        } else if x >= mid_x && y < mid_y {
            dp[mid_y - 1][mid_x - 1] = *l;
            // dp[mid_y - 1][mid_x] = l;
            dp[mid_y][mid_x - 1] = *l;
            dp[mid_y][mid_x] = *l;
            if n >= 4 {
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y + s / 2, mid_x, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y - s / 2, x, y - 1, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x - s / 2, mid_y + s / 2, mid_x - 1, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(
                    n / 2,
                    mid_x - s / 2,
                    mid_y - s / 2,
                    mid_x - 1,
                    mid_y - 1,
                    l,
                    dp,
                );
            }
        } else if x < mid_x && y >= mid_y {
            dp[mid_y - 1][mid_x - 1] = *l;
            dp[mid_y - 1][mid_x] = *l;
            // dp[mid_y][mid_x - 1] = l;
            dp[mid_y][mid_x] = *l;
            if n >= 4 {
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y + s / 2, mid_x, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y - s / 2, mid_x, mid_y - 1, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x - s / 2, mid_y + s / 2, x, y, l, dp);
                *l = *l + 1;
                Solution::cover(
                    n / 2,
                    mid_x - s / 2,
                    mid_y - s / 2,
                    mid_x - 1,
                    mid_y - 1,
                    l,
                    dp,
                );
            }
        } else if x < mid_x && y < mid_y {
            // dp[mid_y - 1][mid_x - 1] = l;
            dp[mid_y - 1][mid_x] = *l;
            dp[mid_y][mid_x - 1] = *l;
            dp[mid_y][mid_x] = *l;
            if n >= 4 {
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y + s / 2, mid_x, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x + s / 2, mid_y - s / 2, mid_x, mid_y - 1, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x - s / 2, mid_y + s / 2, mid_x - 1, mid_y, l, dp);
                *l = *l + 1;
                Solution::cover(n / 2, mid_x - s / 2, mid_y - s / 2, x, y, l, dp);
            }
        }
    }
}

fn main() {
    let n = 16;
    let x = 15;
    let y = 15;
    let mut l = 1;

    let mut dp = vec![vec![0; n]; n];
    dp[y][x] = -1;

    for i in 0..dp.len() {
        println!("{:?}", dp[dp.len() - 1 - i]);
    }

    Solution::cover(n, n / 2, n / 2, x, y, &mut l, &mut dp);

    println!("Result\n\n");
    dp[y][x] = 0;
    for i in 0..dp.len() {
        println!("{:?}", dp[dp.len() - 1 - i]);
    }
}
