use crate::*;

fn check_output(num: String, target: i32, mut expect: Vec<String>) {
    let mut output = Solution::add_operators(num, target);
    output.sort();
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_1() {
    check_output(s!("123"), 6, sv!["1+2+3", "1*2*3"]);
}

#[test]
fn case_2() {
    check_output(s!("232"), 8, sv!["2*3+2", "2+3*2"]);
}

#[test]
fn case_3() {
    check_output(s!("105"), 5, sv!["1*0+5", "10-5"]);
}

#[test]
fn case_4() {
    check_output(s!("00"), 0, sv!["0+0", "0-0", "0*0"]);
}

#[test]
fn case_5() {
    check_output(s!("3456237490"), 9191, sv![]);
}
