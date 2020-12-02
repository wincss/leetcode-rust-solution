use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_primes(0), 0);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_primes(2), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::count_primes(10), 4);
}

#[test]
fn case_4() {
    assert_eq!(Solution::count_primes(10000), 1229);
}

#[test]
fn case_5() {
    assert_eq!(Solution::count_primes(5000000), 348513);
}
