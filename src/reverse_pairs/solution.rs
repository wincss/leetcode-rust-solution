use crate::*;

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        fn bisect_left<T: PartialEq + PartialOrd>(a: &[T], x: T) -> usize {
            let mut l = 0;
            let mut r = a.len();
            while l < r {
                let m = (l + r) / 2;
                if a[m] < x {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            return l;
        }
        let mut s = vec![];
        let mut ans = 0;
        for i in nums.into_iter().rev() {
            let pos = bisect_left(&s, i);
            ans += pos;
            // TODO: trick
            let inpos = bisect_left(&s, i.saturating_mul(2));
            s.insert(inpos, i.saturating_mul(2));
            // println!("{},{},{:?}", pos, inpos, s);
        }
        ans as i32
    }
}
