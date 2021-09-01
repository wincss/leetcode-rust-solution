use crate::*;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        fn split_version(v: String) -> Vec<i32> {
            let mut result = vec![];
            for s in v.split('.') {
                result.push(s.parse().unwrap());
            }
            result
        }
        let mut v1 = split_version(version1).into_iter();
        let mut v2 = split_version(version2).into_iter();
        loop {
            match (v1.next(), v2.next()) {
                (None, None) => return 0,
                (a, b) => match a.unwrap_or(0).cmp(&b.unwrap_or(0)) {
                    std::cmp::Ordering::Less => return -1,
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => return 1,
                },
            }
        }
    }
}
