use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryTrie {
    pub child: [Option<Rc<RefCell<BinaryTrie>>>; 2],
}

impl BinaryTrie {
    #[inline]
    pub fn new() -> Self {
        Self {
            child: [None, None],
        }
    }
}

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        let m = queries.len();
        let mut q: Vec<_> = (0..m).collect();
        q.sort_by_key(|v| queries[*v][1]);

        let t = Rc::new(RefCell::new(BinaryTrie::new()));
        let mut result = vec![-1; m];
        let mut idx = 0;
        for i in q {
            while idx < n && nums[idx] <= queries[i][1] {
                let mut current = Rc::clone(&t);
                for j in (0..32).rev() {
                    let key = (nums[idx] & (1 << j) == 0) as usize;
                    current.borrow_mut().child[key]
                        .get_or_insert(Rc::new(RefCell::new(BinaryTrie::new())));
                    let child = Rc::clone(current.borrow().child[key].as_ref().unwrap());
                    current = child;
                }
                idx += 1;
            }
            let mut max_xor = 0;
            let mut current = Rc::clone(&t);
            for j in (0..32).rev() {
                if current.borrow().child == [None, None] {
                    max_xor = -1;
                    break;
                }
                let mut key = (queries[i][0] & (1 << j) != 0) as usize;
                max_xor <<= 1;
                if current.borrow().child[key].is_none() {
                    key = 1 - key;
                } else {
                    max_xor += 1;
                }
                let child = Rc::clone(current.borrow().child[key].as_ref().unwrap());
                current = child;
            }
            result[i] = max_xor;

            // println!("{:?}", t);
        }
        result
    }
}
