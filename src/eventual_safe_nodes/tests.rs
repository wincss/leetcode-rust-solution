use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![]
        ]),
        vec![2, 4, 5, 6]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::eventual_safe_nodes(vec![
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![3, 4],
            vec![0, 4],
            vec![]
        ]),
        vec![4]
    );
}
