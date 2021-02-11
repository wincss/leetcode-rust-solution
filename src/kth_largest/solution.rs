use std::cmp::Reverse;

#[allow(dead_code)]
pub struct KthLargest {
    k: usize,
    data: Vec<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
impl KthLargest {
    #[allow(dead_code)]
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut n = Self {
            k: k as usize,
            data: vec![],
        };
        for i in nums {
            n.add(i);
        }
        n
    }

    #[allow(dead_code)]
    pub fn add(&mut self, val: i32) -> i32 {
        let idx = self.data.binary_search(&Reverse(val)).unwrap_or_else(|x| x);
        self.data.insert(idx, Reverse(val));
        while self.data.len() > self.k {
            self.data.pop();
        }
        self.data.get(self.k - 1).map(|x| x.0).unwrap_or(0)
    }
}
