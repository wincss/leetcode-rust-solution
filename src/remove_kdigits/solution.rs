use crate::*;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut st = vec![];
        let mut k = k;
        for c in num.chars() {
            while k > 0 && st.last().map_or(false, |v| *v > c) {
                st.pop();
                k -= 1;
            }
            st.push(c);
        }
        for _ in 0..k {
            st.pop();
        }
        let ret: String = st.into_iter().skip_while(|v| *v == '0').collect();
        if ret == String::from("") {
            String::from("0")
        } else {
            ret
        }
    }
}
