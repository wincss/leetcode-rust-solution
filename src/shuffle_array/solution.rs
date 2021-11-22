use rand::{prelude::SliceRandom, thread_rng};

pub struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    #[allow(dead_code)]
    pub fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    #[allow(dead_code)]
    pub fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    #[allow(dead_code)]
    pub fn shuffle(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut nums = self.nums.clone();
        nums.shuffle(&mut rng);
        nums
    }
}
