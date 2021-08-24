use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::all_paths_source_target(vv![[1, 2], [3], [3], []]);
    output.sort();
    assert_eq!(output, vv![[0, 1, 3], [0, 2, 3]]);
}

#[test]
fn case_2() {
    let mut output = Solution::all_paths_source_target(vv![[4, 3, 1], [3, 2, 4], [3], [4], []]);
    output.sort();
    assert_eq!(
        output,
        vv![[0, 1, 2, 3, 4], [0, 1, 3, 4], [0, 1, 4], [0, 3, 4], [0, 4]]
    );
}

#[test]
fn case_3() {
    let mut output = Solution::all_paths_source_target(vv![[1], []]);
    output.sort();
    assert_eq!(output, vv![[0, 1]]);
}

#[test]
fn case_4() {
    let mut output = Solution::all_paths_source_target(vv![[1, 2, 3], [2], [3], []]);
    output.sort();
    assert_eq!(output, vv![[0, 1, 2, 3], [0, 2, 3], [0, 3]]);
}

#[test]
fn case_5() {
    let mut output = Solution::all_paths_source_target(vv![[1, 3], [2], [3], []]);
    output.sort();
    assert_eq!(output, vv![[0, 1, 2, 3], [0, 3]]);
}
