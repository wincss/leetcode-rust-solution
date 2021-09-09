use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::full_justify(
            sv![
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification."
            ],
            16
        ),
        sv!["This    is    an", "example  of text", "justification.  "]
    );
}
