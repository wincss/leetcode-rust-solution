use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_triplets(vec![2, 3]), 0);
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_triplets(vec![1, 3, 5, 7, 9]), 3);
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]),
        8
    );
}
