use crate::*;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut st = vec![];
        for c in s.chars() {
            if *st.last().unwrap_or(&'*') == c {
                st.pop();
            } else {
                st.push(c);
            }
        }
        st.into_iter().collect()
    }
}
