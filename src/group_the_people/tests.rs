use crate::*;

use std::collections::HashSet;

fn check_result(input: Vec<i32>) {
    let output = Solution::group_the_people(input.clone());
    let mut members = HashSet::new();
    for group in output {
        let group_size = group.len();
        for member in group {
            assert_eq!(input[member as usize] as usize, group_size);
            members.insert(member);
        }
    }
    assert_eq!(members.len(), input.len());
}
#[test]
fn case_1() {
    check_result(vec![3, 3, 3, 3, 3, 1, 3]);
}

#[test]
fn case_2() {
    check_result(vec![2, 1, 3, 3, 3, 2]);
}
