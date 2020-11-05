use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::sort_by_bits(vec![10000, 10000]),
        vec![10000, 10000]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
        vec![2, 3, 5, 17, 7, 11, 13, 19]
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::sort_by_bits(vec![10, 100, 1000, 10000]),
        vec![10, 100, 10000, 1000]
    );
}
