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
}

fn main() {
    let text1 = "pmjghexybyrgzczy".to_string();
    let text2 = "hafcdqbgncrcbihkd".to_string();

    println!(
        "Longest common subsequence of {} and {} is {}",
        text1.clone(),
        text2.clone(),
        Solution::longest_common_subsequence0(text1, text2)
    );
}
