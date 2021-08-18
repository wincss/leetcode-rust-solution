use crate::*;

#[test]
fn case_1() {
    let mut output = Solution::accounts_merge(vec![
        sv!["John", "johnsmith@mail.com", "john00@mail.com"],
        sv!["John", "johnnybravo@mail.com"],
        sv!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
        sv!["Mary", "mary@mail.com"],
    ]);
    output.sort();
    assert_eq!(
        output,
        vec![
            sv![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            sv!["John", "johnnybravo@mail.com"],
            sv!["Mary", "mary@mail.com"]
        ]
    );
}
