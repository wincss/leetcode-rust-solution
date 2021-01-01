use crate::*;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut available = 0;
        let mut last_flower = -2;
        let l = flowerbed.len() as i32;
        assert!(l > 0);
        for i in 0..l {
            if flowerbed[i as usize] == 1 {
                available += (i - last_flower - 2) / 2;
                last_flower = i;
            }
        }
        available += (l - last_flower - 1) / 2;
        available >= n
    }
}
