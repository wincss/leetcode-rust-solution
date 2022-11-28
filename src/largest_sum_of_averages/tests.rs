use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
        20_f64
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4),
        20.5_f64
    );
}
