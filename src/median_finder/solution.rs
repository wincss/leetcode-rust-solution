use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(dead_code)]
pub struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_num(&mut self, num: i32) {
        if !self.left.is_empty() && num > *self.left.peek().unwrap() {
            self.right.push(Reverse(num));
        } else {
            self.left.push(num);
        }
        while self.left.len() > self.right.len() + 1 {
            self.right.push(Reverse(self.left.pop().unwrap()));
        }
        while self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
    #[allow(dead_code)]
    pub fn find_median(&self) -> f64 {
        if self.left.len() == self.right.len() {
            (*self.left.peek().unwrap() + self.right.peek().unwrap().0) as f64 / 2_f64
        } else {
            *self.left.peek().unwrap() as f64
        }
    }
}
