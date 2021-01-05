use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::large_group_positions(String::from("abbxxxxzzy")),
        vec![vec![3, 6]]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::large_group_positions(String::from("abc")),
        Vec::<Vec<i32>>::new()
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::large_group_positions(String::from("abcdddeeeeaabbbcd")),
        vec![vec![3, 5], vec![6, 9], vec![12, 14]]
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::large_group_positions(String::from("aba")),
        Vec::<Vec<i32>>::new()
    );
}
