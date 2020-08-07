use crate::ListNode;
#[test]
#[ignore]
fn create_listnode() {
    // build ListNode
    let third = ListNode::new(3);
    let second = ListNode {
        val: 2,
        next: Some(Box::new(third)),
    };
    let first = ListNode {
        val: 1,
        next: Some(Box::new(second)),
    };

    assert_eq!(ListNode::from_array(&[1, 2, 3]), Some(Box::new(first)));
}

#[test]
#[ignore]
fn listnode_to_vec() {
    let head = ListNode::from_array(&[1, 2, 3]);
    assert_eq!(head.unwrap().to_vec(), vec![1, 2, 3]);
}

#[test]
#[ignore]
fn display_listnode() {
    let head = ListNode::from_array(&[1, 2, 3]);
    assert_eq!(format!("{}", head.unwrap()), "[1, 2, 3]".to_string());
}
