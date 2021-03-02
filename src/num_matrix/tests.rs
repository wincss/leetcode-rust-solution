use super::solution::*;

#[test]
fn case_1() {
    let v = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);

    assert_eq!(v.sum_region(2, 1, 4, 3), 8);
    assert_eq!(v.sum_region(1, 1, 2, 2), 11);
    assert_eq!(v.sum_region(1, 2, 2, 4), 12);
}
