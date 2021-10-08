use crate::*;

fn check_output(input: String, mut expect: Vec<String>) {
    let mut output = Solution::find_repeated_dna_sequences(input);
    output.sort();
    expect.sort();
    assert_eq!(output, expect);
}

#[test]
fn case_1() {
    check_output(
        s!("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"),
        sv!["AAAAACCCCC", "CCCCCAAAAA"],
    );
}

#[test]
fn case_2() {
    check_output(s!("AAAAAAAAAAAAA"), sv!["AAAAAAAAAA"]);
}
