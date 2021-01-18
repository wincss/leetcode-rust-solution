use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::accounts_merge(vec![
        to_string_vec(&["John", "johnsmith@mail.com", "john00@mail.com"]),
        to_string_vec(&["John", "johnnybravo@mail.com"]),
        to_string_vec(&["John", "johnsmith@mail.com", "john_newyork@mail.com"]),
        to_string_vec(&["Mary", "mary@mail.com"]),
    ]);
    output.sort();
    assert_eq!(
        output,
        vec![
            to_string_vec(&[
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ]),
            to_string_vec(&["John", "johnnybravo@mail.com"]),
            to_string_vec(&["Mary", "mary@mail.com"])
        ]
    );
}
