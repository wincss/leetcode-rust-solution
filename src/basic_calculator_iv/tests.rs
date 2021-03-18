use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::basic_calculator_iv("e + 8 - a + 5".to_string(), to_string_vec(&["e"]), vec![1]),
        to_string_vec(&["-1*a", "14"])
    );
}
#[test]
fn case_2() {
    assert_eq!(
        Solution::basic_calculator_iv(
            "e - 8 + temperature - pressure".to_string(),
            to_string_vec(&["e", "temperature"]),
            vec![1, 12]
        ),
        to_string_vec(&["-1*pressure", "5"])
    );
}
#[test]
fn case_3() {
    assert_eq!(
        Solution::basic_calculator_iv("(e + 8) * (e - 8)".to_string(), vec![], vec![]),
        to_string_vec(&["1*e*e", "-64"])
    );
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::basic_calculator_iv("7 - 7".to_string(), vec![], vec![]),
        Vec::<String>::new()
    );
}
#[test]
fn case_5() {
    assert_eq!(
        Solution::basic_calculator_iv("a * b * c + b * a * c * 4".to_string(), vec![], vec![]),
        to_string_vec(&["5*a*b*c"])
    );
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::basic_calculator_iv(
            "((a - b) * (b - c) + (c - a)) * ((a - b) + (b - c) * (c - a))".to_string(),
            vec![],
            vec![]
        ),
        to_string_vec(&[
            "-1*a*a*b*b",
            "2*a*a*b*c",
            "-1*a*a*c*c",
            "1*a*b*b*b",
            "-1*a*b*b*c",
            "-1*a*b*c*c",
            "1*a*c*c*c",
            "-1*b*b*b*c",
            "2*b*b*c*c",
            "-1*b*c*c*c",
            "2*a*a*b",
            "-2*a*a*c",
            "-2*a*b*b",
            "2*a*c*c",
            "1*b*b*b",
            "-1*b*b*c",
            "1*b*c*c",
            "-1*c*c*c",
            "-1*a*a",
            "1*a*b",
            "1*a*c",
            "-1*b*c"
        ])
    );
}
