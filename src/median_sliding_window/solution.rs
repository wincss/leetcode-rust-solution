use crate::*;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        assert!(k > 0);
        let mut result = vec![];
        let mut left: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut right: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut left_num = 0;
        let mut right_num = 0;
        for (i, &v) in nums.iter().enumerate() {
            // println!("i={}, v={}, leftmost={}", i, v, (i + 1).saturating_sub(k));
            // println!(
            //     "before left={:?} ({}), right={:?} ({})",
            //     left, left_num, right, right_num
            // );
            if i >= k {
                if (*left.peek().unwrap()).0 >= nums[i - k] {
                    left_num -= 1;
                // println!("remove {} from left", nums[i - k]);
                } else {
                    right_num -= 1;
                    // println!("remove {} from right", nums[i - k]);
                }
            }
            while let Some(&(_, j)) = left.peek() {
                if j + k <= i {
                    left.pop();
                } else {
                    break;
                }
            }
            // println!("left max={:?}", left.peek());
            if !left.is_empty() && (*left.peek().unwrap()).0 > v {
                left.push((v, i));
                left_num += 1;
            } else {
                right.push(Reverse((v, i)));
                right_num += 1;
            }
            // println!(
            //     "after push left={:?} ({}), right={:?} ({})",
            //     left, left_num, right, right_num
            // );
            while left_num < right_num {
                while let Some(Reverse((v, j))) = right.pop() {
                    if j + k <= i {
                        continue;
                    }
                    left.push((v, j));
                    right_num -= 1;
                    left_num += 1;
                    break;
                }
            }
            while left_num > right_num + 1 {
                while let Some((v, j)) = left.pop() {
                    if j + k <= i {
                        continue;
                    }
                    right.push(Reverse((v, j)));
                    left_num -= 1;
                    right_num += 1;
                    break;
                }
            }
            // println!(
            //     "after balance left={:?} ({}), right={:?} ({})",
            //     left, left_num, right, right_num
            // );
            while let Some(&(_, j)) = left.peek() {
                if j + k <= i {
                    left.pop();
                } else {
                    break;
                }
            }
            while let Some(&Reverse((_, j))) = right.peek() {
                if j + k <= i {
                    right.pop();
                } else {
                    break;
                }
            }
            // println!(
            //     "after clear left={:?} ({}), right={:?} ({})",
            //     left, left_num, right, right_num
            // );
            if i + 1 < k {
                continue;
            }
            if k & 1 == 1 {
                result.push((*left.peek().unwrap()).0 as f64);
            } else {
                result.push(
                    ((*left.peek().unwrap()).0 as f64 + ((*right.peek().unwrap()).0).0 as f64)
                        / 2.0,
                );
            }
        }

        result
    }
}
