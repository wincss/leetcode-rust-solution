use crate::*;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut count = 1;
        for c in preorder.split(',') {
            count -= 1;
            if count < 0 {
                return false;
            }
            match c {
                "#" => continue,
                _ => count += 2,
            }
        }
        count == 0
    }
}
