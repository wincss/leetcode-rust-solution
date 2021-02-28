use super::solution::*;

#[test]
fn case_1() {
    let v = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(v.sum_range(0, 2), 1);
    assert_eq!(v.sum_range(2, 5), -1);
    assert_eq!(v.sum_range(0, 5), -3);
}
