use crate::*;

use super::solution::*;

#[test]
fn case_1() {
    let mut dict = WordDictionary::new();
    dict.add_word(s!("bad"));
    dict.add_word(s!("dad"));
    dict.add_word(s!("mad"));
    assert!(!dict.search(s!("pad")));
    assert!(dict.search(s!("bad")));
    assert!(dict.search(s!(".ad")));
    assert!(dict.search(s!("b..")));
}
