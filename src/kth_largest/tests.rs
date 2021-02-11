use super::solution::*;

#[test]
fn case_1() {
    let mut v = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(v.add(3), 4);
    assert_eq!(v.add(5), 5);
    assert_eq!(v.add(10), 5);
    assert_eq!(v.add(9), 8);
    assert_eq!(v.add(4), 8);
}
