use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::beauty_sum(String::from("aabcb")), 5);
}

#[test]
fn case_2() {
    assert_eq!(Solution::beauty_sum(String::from("aabcbaa")), 17);
}

#[test]
fn case_3() {
    assert_eq!(Solution::beauty_sum(String::from(concat!(
            "blwdfjnhvabjuhvzawmkkfvajucwzfknamfhchfcgpbjtlpkaftjebhbyhyorhvtrmrwxvbmipijdxpegmvwpgrlwd",
            "kydhwziaveadbkjpmqkyaeuhgsvtyxhfbngklmnlhcpftgbyuybgyobeyuhcgjwpjcxhvlhhggocprrmaxmjemotst",
            "vrlxauhguyrfyqyicsybanavfmymmajbqshtjpaqoervtlfutxwuwrnlbzgzqipnwflkzcbitbkpubaukvhwfjuzck",
            "gnbsnqqgkmhsexapmdidcxbfgbrmilsgrfqmrfcinlvgcgopazhxwpsrvxelenaxaskmdqwjjivmzijeupkwqfotoj",
            "upxfxgtdgqschlcobevepzopsxvotvoloyqlzmyvhjduiwkuptkhzutudmqojjiyfitvlbyime"
        ))),670774);
}
