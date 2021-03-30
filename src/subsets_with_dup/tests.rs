use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::subsets_with_dup(vec![1, 2, 2]);
    for item in output.iter_mut() {
        item.sort();
    }
    output.sort();
    assert_eq!(
        output,
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    );
}

#[test]
fn case_2() {
    let mut output = Solution::subsets_with_dup(vec![0]);
    for item in output.iter_mut() {
        item.sort();
    }
    output.sort();
    assert_eq!(output, vec![vec![], vec![0]]);
}
