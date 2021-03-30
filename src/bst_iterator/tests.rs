use super::solution::*;
use crate::*;

use serde_json::json;

#[test]
fn case_1() {
    let tree = TreeNode::from_json(json!([7, 3, 15, null, null, 9, 20]));
    let mut iter = BSTIterator::new(tree);
    assert_eq!(iter.next(), 3);
    assert_eq!(iter.next(), 7);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 9);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 15);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 20);
    assert!(!iter.has_next());
}
