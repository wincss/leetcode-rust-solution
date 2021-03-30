use crate::*;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut count: Vec<[i32; 2]> = vec![];
        for i in nums {
            if count.last().is_some() && count.last().unwrap()[0] == i {
                count.last_mut().unwrap()[1] += 1;
            } else {
                count.push([i, 1]);
            }
        }
        let mut current = vec![];
        let mut result = vec![];
        fn make(candidate: &[[i32; 2]], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if candidate.is_empty() {
                result.push(current.clone());
                return;
            }
            for i in 0..=candidate[0][1] {
                for _ in 0..i {
                    current.push(candidate[0][0]);
                }
                make(&candidate[1..], current, result);
                for _ in 0..i {
                    current.pop();
                }
            }
        }
        make(&count[..], &mut current, &mut result);
        result
    }
}
