use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        #[inline]
        fn bucket_id(num: i64, bucket_size: i64) -> i64 {
            if num >= 0 {
                num / bucket_size
            } else {
                (num - bucket_size + 1) / bucket_size
            }
        }
        let mut bucket: HashMap<i64, i64> = HashMap::new();
        let n = nums.len();
        let t = t as i64;
        let k = k as usize;
        let bucket_size = t + 1;
        for i in 0..n {
            let v = nums[i] as i64;
            let bid = bucket_id(v, bucket_size);
            if bucket.contains_key(&bid) {
                return true;
            }
            if bucket
                .get(&(bid - 1))
                .map(|&c| (c - v).abs() <= t)
                .unwrap_or(false)
            {
                return true;
            }
            if bucket
                .get(&(bid + 1))
                .map(|&c| (c - v).abs() <= t)
                .unwrap_or(false)
            {
                return true;
            }
            bucket.insert(bid, v);
            if i >= k {
                bucket.remove(&bucket_id(nums[i - k] as i64, bucket_size));
            }
        }
        false
    }
}
