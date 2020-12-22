use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reformat_number(String::from("1-23-45 6")),
        String::from("123-456")
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::reformat_number(String::from("123 4-567")),
        String::from("123-45-67")
    )
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::reformat_number(String::from("123 4-5678")),
        String::from("123-456-78")
    )
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::reformat_number(String::from("12")),
        String::from("12")
    )
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::reformat_number(String::from("--17-5 229 35-39475 ")),
        String::from("175-229-353-94-75")
    )
}
