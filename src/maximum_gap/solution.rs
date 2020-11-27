use crate::*;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        let maxn = nums.iter().map(|&v| v).max().unwrap();
        let minn = nums.iter().map(|&v| v).min().unwrap();
        let interval = std::cmp::max(1, (maxn - minn) / (n - 1) as i32);
        let bnum = ((maxn - minn) / interval + 1) as usize;
        let mut bmax = vec![std::i32::MIN; bnum];
        let mut bmin = vec![std::i32::MAX; bnum];
        for i in nums.into_iter() {
            let bid = ((i - minn) / interval) as usize;
            bmax[bid] = std::cmp::max(bmax[bid], i);
            bmin[bid] = std::cmp::min(bmin[bid], i);
        }
        let mut ret = 0;
        let mut last = 0;
        for i in 1..bnum {
            if bmax[i] > std::i32::MIN {
                // println!("{},{},{}", bmax[i], bmin[i], bmax[last]);
                ret = std::cmp::max(ret, bmin[i] - bmax[last]);
                last = i;
            }
        }
        ret
    }
}
