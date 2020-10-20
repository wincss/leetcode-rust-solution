use crate::*;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut typed_iter = typed.chars();
        let mut last = None;
        let mut ch;
        for i in name.chars() {
            loop {
                ch = typed_iter.next();
                if ch == Some(i) {
                    break;
                } else if ch == last {
                    continue;
                } else {
                    return false;
                }
            }
            last = ch;
        }
        while let Some(v) = typed_iter.next() {
            if Some(v) != last {
                return false;
            }
        }
        true
    }
}
