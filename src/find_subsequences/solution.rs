use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(
            nums: &Vec<i32>,
            i: usize,
            big: Option<usize>,
            save: &mut HashMap<(usize, Option<usize>), HashSet<Vec<i32>>>,
        ) -> HashSet<Vec<i32>> {
            let n = nums.len();
            if n == 0 {
                return HashSet::new();
            }
            if i == n {
                let mut h = HashSet::new();
                h.insert(vec![]);
                return h;
            }
            if let Some(ans) = save.get(&(i, big)) {
                return ans.clone();
            }
            let mut ans = helper(nums, i + 1, big, save);
            if big.is_none() || nums[i] >= nums[big.unwrap()] {
                let next = helper(nums, i + 1, Some(i), save)
                    .iter()
                    .map(|v| {
                        vec![nums[i]]
                            .into_iter()
                            .chain(v.clone().into_iter())
                            .collect()
                    })
                    .collect::<HashSet<Vec<i32>>>();
                ans.extend(next);
            }
            save.insert((i, big), ans.clone());
            ans
        }
        let mut save = HashMap::new();
        helper(&nums, 0, None, &mut save)
            .into_iter()
            .filter(|v| v.len() >= 2)
            .collect()
    }
}
