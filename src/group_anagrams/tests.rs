use crate::*;

#[test]
fn case_1() {
    let mut output =
        Solution::group_anagrams(to_string_vec(&["eat", "tea", "tan", "ate", "nat", "bat"]));
    for item in output.iter_mut() {
        item.sort();
    }
    output.sort();
    assert_eq!(
        output,
        vec![
            to_string_vec(&["ate", "eat", "tea"]),
            to_string_vec(&["bat"]),
            to_string_vec(&["nat", "tan"]),
        ]
    );
}
