use crate::*;

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
enum State {
    Installed,
    Monitored,
    Unmonitored,
}
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn cover(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> State {
            if root.is_none() {
                return State::Monitored;
            }
            let root = root.as_ref().unwrap().borrow();
            let left_state = cover(root.left.as_ref().map(Rc::clone), result);
            let right_state = cover(root.right.as_ref().map(Rc::clone), result);
            match (left_state, right_state) {
                (State::Unmonitored, _) | (_, State::Unmonitored) => {
                    *result += 1;
                    State::Installed
                }
                (State::Monitored, State::Monitored) => State::Unmonitored,
                (State::Installed, _) | (_, State::Installed) => State::Monitored,
            }
        }
        let mut result = 0;
        if cover(root, &mut result) == State::Unmonitored {
            result += 1;
        }
        result
    }
}
