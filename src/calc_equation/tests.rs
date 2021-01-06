use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::calc_equation(
            vec![to_string_vec(&["a", "b"]), to_string_vec(&["b", "c"])],
            vec![2.0, 3.0],
            vec![
                to_string_vec(&["a", "c"]),
                to_string_vec(&["b", "a"]),
                to_string_vec(&["a", "e"]),
                to_string_vec(&["a", "a"]),
                to_string_vec(&["x", "x"])
            ]
        ),
        vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::calc_equation(
            vec![
                to_string_vec(&["a", "b"]),
                to_string_vec(&["b", "c"]),
                to_string_vec(&["bc", "cd"])
            ],
            vec![1.5, 2.5, 5.0],
            vec![
                to_string_vec(&["a", "c"]),
                to_string_vec(&["c", "b"]),
                to_string_vec(&["bc", "cd"]),
                to_string_vec(&["cd", "bc"]),
            ]
        ),
        vec![3.75000, 0.40000, 5.00000, 0.20000]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::calc_equation(
            vec![to_string_vec(&["a", "b"]),],
            vec![0.5],
            vec![
                to_string_vec(&["a", "b"]),
                to_string_vec(&["b", "a"]),
                to_string_vec(&["a", "c"]),
                to_string_vec(&["x", "y"]),
            ]
        ),
        vec![0.50000, 2.00000, -1.00000, -1.00000]
    );
}
