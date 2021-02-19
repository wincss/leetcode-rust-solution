use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
        6
    );
}
