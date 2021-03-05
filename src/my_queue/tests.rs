use super::solution::*;

#[test]
fn case_1() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.peek(), 1);
    assert_eq!(q.pop(), 1);
    assert!(!q.empty());
}
