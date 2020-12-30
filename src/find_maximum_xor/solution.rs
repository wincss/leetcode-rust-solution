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
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let t = Rc::new(RefCell::new(BinaryTrie::new()));
        let mut result = 0;
        for i in nums {
            let mut max_xor = 0;
            let mut current = Rc::clone(&t);
            for j in (0..32).rev() {
                if current.borrow().child == [None, None] {
                    break;
                }
                let mut key = (i & (1 << j) != 0) as usize;
                max_xor <<= 1;
                if current.borrow().child[key].is_none() {
                    key = 1 - key;
                } else {
                    max_xor += 1;
                }
                let child = Rc::clone(current.borrow().child[key].as_ref().unwrap());
                current = child;
            }
            result = std::cmp::max(max_xor, result);
            let mut current = Rc::clone(&t);
            for j in (0..32).rev() {
                let key = (i & (1 << j) == 0) as usize;
                current.borrow_mut().child[key]
                    .get_or_insert(Rc::new(RefCell::new(BinaryTrie::new())));
                let child = Rc::clone(current.borrow().child[key].as_ref().unwrap());
                current = child;
            }
            // println!("{:?}", t);
        }
        result
    }
}
