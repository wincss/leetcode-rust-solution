use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    output.sort();
    assert_eq!(output, vec![1, 2]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::top_k_frequent(
            to_string_vec(&["i", "love", "leetcode", "i", "love", "coding"]),
            2
        ),
        to_string_vec(&["i", "love"])
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::top_k_frequent(
            to_string_vec(&["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"]),
            4
        ),
        to_string_vec(&["the", "is", "sunny", "day"])
    );
}
