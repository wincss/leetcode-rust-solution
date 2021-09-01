use crate::*;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split('.').map(|x| x.parse::<i32>().unwrap());
        let mut v2 = version2.split('.').map(|x| x.parse::<i32>().unwrap());
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
