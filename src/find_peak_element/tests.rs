use crate::*;

fn check_result(input: Vec<i32>) {
    let result = Solution::find_peak_element(input.clone()) as usize;
    if result == 0 {
        assert!(input[1] < input[0]);
    } else if result == input.len() - 1 {
        assert!(input[result] > input[result - 1]);
    } else {
        assert!(input[result] > input[result - 1] && input[result] > input[result + 1]);
    }
}

#[test]
fn case_1() {
    check_result(vv![1, 2, 3, 1]);
}

#[test]
fn case_2() {
    check_result(vv![1, 2, 1, 3, 5, 6, 4]);
}
