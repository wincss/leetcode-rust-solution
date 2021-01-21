use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::find_critical_and_pseudo_critical_edges(
        5,
        vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![2, 3, 2],
            vec![0, 3, 2],
            vec![0, 4, 3],
            vec![3, 4, 3],
            vec![1, 4, 6],
        ],
    );
    for v in output.iter_mut() {
        v.sort();
    }
    assert_eq!(output, vec![vec![0, 1], vec![2, 3, 4, 5]]);
}

#[test]
fn case_2() {
    let mut output = Solution::find_critical_and_pseudo_critical_edges(
        4,
        vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]],
    );
    for v in output.iter_mut() {
        v.sort();
    }
    assert_eq!(output, vec![vec![], vec![0, 1, 2, 3]]);
}

#[test]
fn case_3() {
    let mut output = Solution::find_critical_and_pseudo_critical_edges(
        6,
        vec![
            vec![0, 1, 1],
            vec![1, 2, 1],
            vec![0, 2, 1],
            vec![2, 3, 4],
            vec![3, 4, 2],
            vec![3, 5, 2],
            vec![4, 5, 2],
        ],
    );
    for v in output.iter_mut() {
        v.sort();
    }
    assert_eq!(output, vec![vec![3], vec![0, 1, 2, 4, 5, 6]]);
}

#[test]
fn case_4() {
    let mut output = Solution::find_critical_and_pseudo_critical_edges(
        6,
        vec![
            vec![0, 1, 2],
            vec![0, 2, 5],
            vec![2, 3, 5],
            vec![1, 4, 4],
            vec![2, 5, 5],
            vec![4, 5, 2],
        ],
    );
    for v in output.iter_mut() {
        v.sort();
    }
    assert_eq!(output, vec![vec![0, 2, 3, 5], vec![1, 4]]);
}
