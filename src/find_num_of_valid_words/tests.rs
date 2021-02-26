use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_num_of_valid_words(
            to_string_vec(&["aaaa", "asas", "able", "ability", "actt", "actor", "access"]),
            to_string_vec(&["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"])
        ),
        vec![1, 1, 3, 2, 4, 0]
    );
}
