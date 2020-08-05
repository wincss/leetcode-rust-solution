use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn maketree(n: i32, offset: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if n == 0 {
                return vec![None];
            }
            let mut ans = Vec::new();
            for i in 0..n {
                for l in maketree(i, offset).iter() {
                    for mut r in maketree(n - 1 - i, offset + i + 1).into_iter() {
                        let mut root = TreeNode::new(i + offset);
                        root.left = l.as_ref().and_then(|v| Some(Rc::clone(v)));
                        root.right = r.take();
                        ans.push(Some(Rc::new(RefCell::new(root))));
                    }
                }
            }
            ans
        }
        if n == 0 {
            vec![]
        } else {
            maketree(n, 1)
        }
    }
}
