use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
        vec![vec![-2, 2]]
    );
}

#[test]
fn case_2() {
    let mut output = Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
    output.sort();
    assert_eq!(output, vec![vec![-2, 4], vec![3, 3]]);
}
