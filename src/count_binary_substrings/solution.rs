use crate::*;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut this = 0;
        let mut last_count = 0;
        let mut this_count = 0;
        for &i in s.as_bytes().into_iter() {
            if i != this {
                ans += std::cmp::min(last_count, this_count);
                last_count = this_count;
                this_count = 1;
                this = i;
            } else {
                this_count += 1;
            }
        }
        ans += std::cmp::min(last_count, this_count);
        ans
    }
}
