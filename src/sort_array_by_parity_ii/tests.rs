use crate::*;

fn check_result(input: Vec<i32>) {
    let mut output = Solution::sort_array_by_parity_ii(input.clone());
    for (i, &v) in output.iter().enumerate() {
        assert!((i as i32 ^ v) & 1 == 0);
    }
    let mut input = input;
    input.sort();
    output.sort();
    assert_eq!(input, output);
}
#[test]
fn case_1() {
    check_result(vec![4, 2, 5, 7]);
}
