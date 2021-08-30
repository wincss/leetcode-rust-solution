use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::number_of_unique_good_subsequences(s!("001")), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::number_of_unique_good_subsequences(s!("11")), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::number_of_unique_good_subsequences(s!("101")), 5);
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::number_of_unique_good_subsequences(s!("1110001")),
        23
    );
}
