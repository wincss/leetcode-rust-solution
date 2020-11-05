use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::find_ladders(
        String::from("hit"),
        String::from("cog"),
        to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]),
    );
    output.sort();
    assert_eq!(
        output,
        vec![
            to_string_vec(&["hit", "hot", "dot", "dog", "cog"]),
            to_string_vec(&["hit", "hot", "lot", "log", "cog"])
        ]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_ladders(
            String::from("hit"),
            String::from("cog"),
            to_string_vec(&["hot", "dot", "dog", "lot", "log"])
        ),
        Vec::<Vec<String>>::new()
    );
}
