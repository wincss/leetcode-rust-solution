#[allow(dead_code)]
pub struct NumArray {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */
impl NumArray {
    #[allow(dead_code)]
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = vec![0];
        for i in nums {
            prefix_sum.push(prefix_sum.last().unwrap() + i);
        }
        Self { prefix_sum }
    }

    #[allow(dead_code)]
    pub fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.prefix_sum[j as usize + 1] - self.prefix_sum[i as usize]
    }
}
