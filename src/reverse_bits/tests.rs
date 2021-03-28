use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::reverse_bits(0b00000010100101000001111010011100),
        0b00111001011110000010100101000000
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::reverse_bits(0b11111111111111111111111111111101),
        0b10111111111111111111111111111111
    );
}
