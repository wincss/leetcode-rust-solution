use crate::*;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = None;
        let mut last_count = 0;
        let mut idx = 0;
        let n = nums.len();
        for i in 0..n {
            if last == Some(nums[i]) && last_count >= 2 {
                continue;
            }
            nums[idx] = nums[i];
            idx += 1;
            if last == Some(nums[i]) {
                last_count += 1;
            } else {
                last_count = 1;
                last = Some(nums[i]);
            }
        }
        idx as i32
    }

    // remove_duplicates in origin problem
    pub fn remove_duplicates_string(s: String) -> String {
        let mut st = vec![];
        for c in s.chars() {
            if *st.last().unwrap_or(&'*') == c {
                st.pop();
            } else {
                st.push(c);
            }
        }
        st.into_iter().collect()
    }
}
