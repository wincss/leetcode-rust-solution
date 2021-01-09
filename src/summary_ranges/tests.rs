use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        to_string_vec(&["0->2", "4->5", "7"])
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        to_string_vec(&["0", "2->4", "6", "8->9"])
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::summary_ranges(vec![]), Vec::<String>::new());
}

#[test]
fn case_4() {
    assert_eq!(Solution::summary_ranges(vec![-1]), to_string_vec(&["-1"]));
}

#[test]
fn case_5() {
    assert_eq!(Solution::summary_ranges(vec![0]), to_string_vec(&["0"]));
}
