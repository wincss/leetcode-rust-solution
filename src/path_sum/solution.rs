use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

impl Solution {
    pub fn path_sum_113(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        fn walk(
            root: Option<Rc<RefCell<TreeNode>>>,
            remain: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if root.is_none() {
                return;
            }
            let mut node = root.as_ref().unwrap().borrow_mut();
            current.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                if node.val == remain {
                    result.push(current.clone());
                }
            } else {
                walk(node.left.take(), remain - node.val, current, result);
                walk(node.right.take(), remain - node.val, current, result);
            }
            current.pop();
        }
        let mut result = vec![];
        let mut current = vec![];
        walk(root, sum, &mut current, &mut result);
        result
    }

    pub fn path_sum_437(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn walk(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i64,
            current_sum: i64,
            prefix: &mut HashMap<i64, i32>,
            count: &mut i32,
        ) {
            if root.is_none() {
                return;
            }
            let node = root.as_ref().unwrap().borrow();
            let current_sum = current_sum + node.val as i64;
            *count += *prefix.get(&(current_sum - target_sum)).unwrap_or(&0);
            *prefix.entry(current_sum).or_insert(0) += 1;
            walk(
                node.left.as_ref().map(Rc::clone),
                target_sum,
                current_sum,
                prefix,
                count,
            );
            walk(
                node.right.as_ref().map(Rc::clone),
                target_sum,
                current_sum,
                prefix,
                count,
            );
            prefix.entry(current_sum).and_modify(|v| *v -= 1);
        }
        let mut count = 0;
        let mut prefix = HashMap::new();
        prefix.insert(0, 1);
        walk(root, target_sum as i64, 0, &mut prefix, &mut count);
        count
    }
}
