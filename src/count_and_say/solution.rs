use crate::*;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 2..=n {
            let last = result;
            result = String::new();
            let mut lc = ' ';
            let mut lcc = 0;
            for c in last.chars() {
                if c == lc {
                    lcc += 1;
                } else {
                    if lcc > 0 {
                        result.push_str(&lcc.to_string());
                        result.push(lc);
                    }
                    lc = c;
                    lcc = 1;
                }
            }
            if lcc > 0 {
                result.push_str(&lcc.to_string());
                result.push(lc);
            }
        }
        result
    }
}
