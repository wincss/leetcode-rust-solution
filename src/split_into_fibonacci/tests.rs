use crate::*;

fn check_output(input: &str, is_valid: bool) {
    let output = Solution::split_into_fibonacci(String::from(input));
    if !is_valid {
        assert_eq!(output, Vec::<i32>::new());
        return;
    }
    let chained: String = output.iter().map(|v| v.to_string()).collect();
    assert_eq!(chained, input);
    let n = output.len();
    assert!(n > 2);
    for i in 2..n {
        assert_eq!(output[i - 2] + output[i - 1], output[i]);
    }
}

#[test]
fn case_1() {
    check_output("123456579", true);
}

#[test]
fn case_2() {
    check_output("11235813", true);
}

#[test]
fn case_3() {
    check_output("112358130", false);
}

#[test]
fn case_4() {
    check_output("0123", false);
}

#[test]
fn case_5() {
    check_output("1012", false);
}

#[test]
fn case_6() {
    check_output("5511816597", false);
}

#[test]
fn case_7() {
    check_output("1101111", true);
}
