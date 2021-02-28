use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_common_subsequence0(text1: String, text2: String) -> i32 {
        Solution::longest_common_subsequence0_impl(&text1, text1.len() - 1, &text2, text2.len() - 1)
    }

    fn longest_common_subsequence0_impl(
        str1: &String,
        index1: usize,
        str2: &String,
        index2: usize,
    ) -> i32 {
        if str1.chars().nth(index1) == str2.chars().nth(index2) {
            if index1 != 0 && index2 != 0 {
                Solution::longest_common_subsequence0_impl(str1, index1 - 1, str2, index2 - 1) + 1
            } else {
                1
            }
        } else {
            if index1 == 0 && index2 == 0 {
                0
            } else if index1 == 0 {
                Solution::longest_common_subsequence0_impl(str1, index1, str2, index2 - 1)
            } else if index2 == 0 {
                Solution::longest_common_subsequence0_impl(str1, index1 - 1, str2, index2)
            } else {
                max(
                    Solution::longest_common_subsequence0_impl(str1, index1, str2, index2 - 1),
                    Solution::longest_common_subsequence0_impl(str1, index1 - 1, str2, index2),
                )
            }
        }
    }

    pub fn longest_common_subsequence1(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for i in 1..(text1.len() + 1) {
            for j in 1..(text2.len() + 1) {
                if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[text1.len()][text2.len()]
    }

    pub fn longest_common_subsequence2(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let n1 = text1.len() + 1;
        let n2 = text2.len() + 1;
        let mut dp = vec![vec![0; n2]; n1];
        for i in 1..n1 {
            for j in 1..n2 {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[n1 - 1][n2 - 1]
    }

    pub fn longest_common_subsequence3(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let n1 = text1.len() + 1;
        let n2 = text2.len() + 1;
        let mut cur = &mut vec![0; n2];
        let mut prv = &mut vec![0; n2];

        for i in 1..n1 {
            for j in 1..n2 {
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prv[j - 1] + 1;
                } else {
                    cur[j] = std::cmp::max(cur[j - 1], prv[j]);
                }
            }
            let tmp = prv;
            prv = cur;
            cur = tmp;
        }
        prv[n2 - 1]
    }

    pub fn longest_common_subsequence4(text1: String, text2: String) -> i32 {
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let n1 = text1.len() + 1;
        let n2 = text2.len() + 1;
        let mut cur = vec![0; n2];
        let mut prv = vec![0; n2];

        for i in 1..n1 {
            for j in 1..n2 {
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prv[j - 1] + 1;
                } else {
                    cur[j] = std::cmp::max(cur[j - 1], prv[j]);
                }
            }
            let tmp = prv;
            prv = cur;
            cur = tmp;
        }
        prv[n2 - 1]
    }

    pub fn longest_common_subsequence5(text1: String, text2: String) -> i32 {
        // 实际上，对于版本4的程序，只有prv[j - 1] 和 prv[j]用到了，可以进一步简化内存
        let s1 = text1.as_bytes();
        let s2 = text2.as_bytes();
        let n1 = text1.len() + 1;
        let n2 = text2.len() + 1;
        let mut cur = vec![0; n2];
        let mut prv;
        let mut pprv;

        for i in 1..n1 {
            prv = 0;
            for j in 1..n2 {
                pprv = cur[j];
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prv + 1;
                } else {
                    cur[j] = std::cmp::max(cur[j - 1], cur[j]);
                }
                prv = pprv;
            }
        }
        cur[n2 - 1]
    }
}

fn main() {
    let text1 = "bsbininm".to_string();
    let text2 = "jmjkbkjkv".to_string();

    println!(
        "Longest common subsequence of {} and {} is {}",
        text1.clone(),
        text2.clone(),
        Solution::longest_common_subsequence5(text1, text2)
    );
}
