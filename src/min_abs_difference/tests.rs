use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_abs_difference(vec![5, -7, 3, 5], 6), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_abs_difference(vec![7, -9, 15, -2], -5), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_abs_difference(vec![1, 2, 3], -7), 7);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::min_abs_difference(
            vec![
                -590, -424, -540, 142, 6, 824, 36, 40, 125, -182, -337, -573, -808, 821, -768,
                -109, 417, -959, 696, 500, 36, 918, 914, 52, -245, 502, -781, -357, -388, 813,
                -563, 362, 499, 609, 324, -931, 402, -630, 250, -589
            ],
            0
        ),
        0
    );
}
