use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn max_product_318(words: Vec<String>) -> i32 {
        let mut masks = HashMap::new();
        for word in words.iter() {
            let mut mask = 0;
            for c in word.chars() {
                mask |= 1 << (c as u8 - 97);
            }
            let v = masks.entry(mask).or_insert(0);
            *v = word.len().max(*v);
        }
        let mut max = 0;
        for (&mask1, &len1) in masks.iter() {
            for (&mask2, &len2) in masks.iter() {
                if mask1 & mask2 == 0 {
                    max = max.max(len1 * len2);
                }
            }
        }
        max as i32
    }
    pub fn max_product_2002(s: String) -> i32 {
        fn check(s: &Vec<char>, n: usize, mask: i32) -> bool {
            let mut left = 0;
            let mut right = n - 1;
            while left < right {
                while (1 << left) & mask == 0 {
                    left += 1;
                }
                while (1 << right) & mask == 0 {
                    right -= 1;
                }
                if left >= right {
                    return true;
                }
                if s[left] != s[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true
        }
        fn longest(s: &Vec<char>, n: usize, mask: i32) -> i32 {
            let mut dp = vec![0; n];
            let mut idx = 0;
            for left in 0..n {
                if (1 << left) & mask == 0 {
                    continue;
                }
                let mut last = 0;
                idx = 0;
                for right in (0..n).rev() {
                    if (1 << right) & mask == 0 {
                        continue;
                    }
                    let tmp = dp[idx];
                    if s[left] == s[right] {
                        dp[idx] = last + 1;
                    } else if idx > 0 {
                        dp[idx] = dp[idx].max(dp[idx - 1]);
                    }
                    last = tmp;
                    idx += 1;
                }
            }
            dp[idx - 1]
        }
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut max = 1;
        let fullmask = (1 << n) - 1;
        for mask in 1..fullmask {
            if check(&s, n, mask) {
                max = max.max(mask.count_ones() as i32 * longest(&s, n, fullmask - mask));
            }
        }
        max
    }
}
