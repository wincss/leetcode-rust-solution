use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        fn collect(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, result: &mut Vec<Vec<f64>>) {
            if root.is_none() {
                return;
            }
            if result.len() <= depth {
                result.push(vec![0_f64, 0_f64]);
            }
            let root = root.as_ref().unwrap().borrow();
            result[depth][0] += root.val as f64;
            result[depth][1] += 1_f64;
            collect(root.left.as_ref().map(Rc::clone), depth + 1, result);
            collect(root.right.as_ref().map(Rc::clone), depth + 1, result)
        }
        let mut result = vec![];
        collect(root, 0, &mut result);
        result
            .into_iter()
            .map(|v| v[0] / v[1])
            .collect::<Vec<f64>>()
    }
}
