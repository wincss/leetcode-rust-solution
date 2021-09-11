use crate::*;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        // Prerequisites: all items in nums is below 30
        const MOD: i64 = 1000000007;
        let prime = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        // numbers without duplicate factors
        let good = vec![
            2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26, 29, 30,
        ];
        let factor = vec![
            1, 2, 4, 3, 8, 5, 16, 32, 9, 6, 64, 128, 10, 17, 256, 33, 512, 7,
        ];
        // prime factors mask of good numbers
        // let mut _factor = vec![0; good.len()];
        // for (i, &v) in good.iter().enumerate() {
        //     for (j, &p) in prime.iter().enumerate() {
        //         if v % p == 0 {
        //             _factor[i] |= 1 << j;
        //         }
        //     }
        // }
        // assert_eq!(factor, _factor);

        let mut count = vec![0; 31];
        for i in nums.into_iter() {
            count[i as usize] += 1;
        }

        let mut s = 0;
        let mut dp = vec![0; 1 << prime.len()];
        dp[0] = 1;
        for (j, &v) in factor.iter().enumerate() {
            for i in 0..1 << prime.len() {
                if i & v == v {
                    let t = (dp[i ^ v] * count[good[j] as usize]) % MOD;
                    dp[i] = (dp[i] + t) % MOD;
                    s = (s + t) % MOD;
                }
            }
        }
        let mut base = 2;
        while count[1] > 0 {
            if count[1] & 1 == 1 {
                s = s * base % MOD;
            }
            count[1] >>= 1;
            base = base * base % MOD;
        }
        (s % MOD) as i32
    }
}
