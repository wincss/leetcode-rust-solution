use super::solution::*;
use crate::*;

#[test]
fn case_1() {
    let mut obj = MapSum::new();
    obj.insert(s!("apple"), 3);
    assert_eq!(obj.sum(s!("ap")), 3);
    obj.insert(s!("app"), 2);
    assert_eq!(obj.sum(s!("ap")), 5);
    obj.insert(s!("apple"), 8);
    assert_eq!(obj.sum(s!("ap")), 10);
}
