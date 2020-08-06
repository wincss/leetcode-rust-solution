use crate::*;

use std::collections::HashSet;
use std::fmt::Display;

fn check_answer<T: Display>(input: &[T], output: Vec<Vec<i32>>) {
    let answer = Solution::palindrome_pairs(to_string_vec(input));
    assert_eq!(output.len(), answer.len());
    let answer: HashSet<Vec<i32>> = answer.into_iter().collect();
    let expect: HashSet<Vec<i32>> = output.into_iter().collect();
    assert_eq!(answer, expect);
}

#[test]
fn case_1() {
    check_answer(
        &["abcd", "dcba", "lls", "s", "sssll"],
        vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]],
    );
}

#[test]
fn case_2() {
    check_answer(&["bat", "tab", "cat"], vec![vec![0, 1], vec![1, 0]]);
}

#[test]
fn case_3() {
    check_answer(&["aaababbbbabaabbbbbaaaababbbaaabbbbbabbbbaaabbabbbbaaaabbbaabbaabbbabbaaabbbbaaabbbaabaaabbabbabaabbabababaabbaabbbbaaaaaabb","bbbbabaaababaabbbbbbbbababaaabaaabbbaabaaabbaaabaabbaababbbbaabbbaabbbbabbbbbaabbababbbbaabbbbbaaaabaabbbbbababbbbababbbbaabbabbbaabaaabbaaaaaababaaaaa","babbb","babbabbbaabaaaabbabababbaaaaabaababbaaaaaaabaabaaabbaaaabba","baaaaabaabbbaabbaababbabbbbabaababbabbbbabbaabbaaabbaaaabbabbbbaaabaaaaaabbbaaabababaabaababbbbaabbaabbababbaabbbaababbbaa","abbbbbbababbaaaaaabbabababababbabbababaaaaababaababbbabaaabbbbabbbbabbababbaababbaab","abababbabbababbaaaaabbaabba","aabaababababaabbbbbbbabaaabaabbbbabbabababbaaaaababbbbbabaabaabbaababbabaabaaaaabbbbbabbaaabbaabbababbaaaaaabababaababaaababaabbabababbabababbbbabaabaaababaaabaababbbbaabbbbabbabbbaaaabbaabbbabbbaba","abbabaaababbabaaababbbbabbaabbaaabbbbbaabbabaababbbbbbababbaabababaababaabbbabbbbbbaababbbbbbbabbbaabbabbbbabbbaababbbbbaaabbbba","baabaaaaaabbbababaabbaababbbbaababbbbbbabababaababaaabababbabbabbaaaaababbbaaaaaaaaabbaaababbabbabbabbaabaabaaabaabaabbaba","baabbabbaabbabbbbbbaababbababbabbaaaaabbbabbaaabbabbbbaaaababbaaaabaaaabbbbbbbaabbbaabaaabbbbaaaabbaaabbbbabbabaabbaaababbbbaababbbaaababbaabaaaaabbbbaabababaaaaabaabbbbaababab","bbbaababbbaabaaabbbbbaab","abaabbbabaaabaabababbababbabbaaabbbaaaaabbabbbabaabaaaabba","ababbabbababbaaaabaaaaabbbababbbaabaaaaaabbaaabbbaababbabaabbabaaaaaabaabbbbabbaabbbbabbaa","bbbbbabbbbabbabaababbabaabaaaaabaabababbbaaaabbaabaabbabbbaabaabbbbaaababaaaabaabbbbaabababbbbabbabaaaa","abaabababbaabababababaabaabaaabaabbaabaaabbabaaabbbaaaababbaaabbababbabaababaabbabbbabaabbaabbbaaababbaababbabbabaabaababbaabbb","bbabaabababbaaababbabaabaaaabababb","abbaabaaaaaaabaabbabbbaaabbaaaabbbbabaabbabbbababbabaaabbbbbabaaaabbbaa","baaababbaaabbbaaabbabababababbaabbbaaaaabaaabbaaababababbaaaaabababbbaaabbaaabaababab","aabaabbabaaaaaaaaaabaaaaabbbabbbababbaabaabbaaabaaabaabaaaaababaabaaaaaaaaababaabbbbabbbbaaabaabbaabbaaaabbbabbbbaaaabaaabbabbbbbababab"], vec![]);
}
