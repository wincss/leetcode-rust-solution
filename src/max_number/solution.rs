use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        fn max_k(num: &Vec<i32>, k: usize) -> Vec<i32> {
            let n = num.len();
            let mut st = vec![];
            let mut can_skip = n - k;
            for &i in num {
                while let Some(&v) = st.last() {
                    if can_skip == 0 || v >= i {
                        break;
                    }
                    st.pop();
                    can_skip -= 1;
                }
                st.push(i);
            }
            while st.len() > k {
                st.pop();
            }
            st
        }
        fn merge(num1: Vec<i32>, num2: Vec<i32>) -> Vec<i32> {
            let mut num1 = &num1[..];
            let mut num2 = &num2[..];
            let mut result = vec![];
            while num1.len() > 0 || num2.len() > 0 {
                if num1.len() == 0 {
                    result.push(num2[0]);
                    num2 = &num2[1..];
                } else if num2.len() == 0 {
                    result.push(num1[0]);
                    num1 = &num1[1..];
                } else if num1 > num2 {
                    result.push(num1[0]);
                    num1 = &num1[1..];
                } else {
                    result.push(num2[0]);
                    num2 = &num2[1..];
                }
            }
            result
        }
        let mut result = vec![];
        let k = k as usize;
        for i in 0..std::cmp::min(nums1.len(), k) + 1 {
            if k - i > nums2.len() {
                continue;
            }
            // println!("{} > {}, {} > {}", nums1.len(), i, nums2.len(), k - i);
            let n1 = max_k(&nums1, i);
            let n2 = max_k(&nums2, k - i);
            // println!("{:?}, {:?}", n1, n2);
            let r = merge(n1, n2);
            // println!("{:?}", r);
            result = std::cmp::max(result, r);
        }
        result
    }

    pub fn max_number_timeout(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        // correct, but timeout
        fn dp(
            remaining: usize,
            nums1: &[i32],
            nums2: &[i32],
            saved: &mut HashMap<(usize, usize, usize), Option<Vec<i32>>>,
        ) -> Option<Vec<i32>> {
            if remaining == 0 {
                return Some(vec![]);
            }
            if nums1.len() + nums2.len() < remaining {
                return None;
            }
            let key = (remaining, nums1.len(), nums2.len());
            if let Some(result) = saved.get(&key) {
                return result.clone();
            }
            let mut result = vec![];
            if nums1.len() > 0 {
                if let Some(mut r) = dp(remaining - 1, &nums1[1..], nums2, saved) {
                    let mut candidate = vec![nums1[0]];
                    candidate.append(&mut r);
                    result = std::cmp::max(result, candidate);
                }
                if let Some(r) = dp(remaining, &nums1[1..], nums2, saved) {
                    result = std::cmp::max(result, r);
                }
            }
            if nums2.len() > 0 {
                if let Some(mut r) = dp(remaining - 1, nums1, &nums2[1..], saved) {
                    let mut candidate = vec![nums2[0]];
                    candidate.append(&mut r);
                    result = std::cmp::max(result, candidate);
                }
                if let Some(r) = dp(remaining, nums1, &nums2[1..], saved) {
                    result = std::cmp::max(result, r);
                }
            }
            // println!(
            //     "k={}, n1={:?}, n2={:?}, r={:?}",
            //     remaining, nums1, nums2, result
            // );
            saved.insert(
                key,
                if result.len() == 0 {
                    None
                } else {
                    Some(result)
                },
            );
            saved[&key].clone()
        }
        let mut saved = HashMap::new();
        dp(k as usize, &nums1[..], &nums2[..], &mut saved).unwrap()
    }
}
