use crate::*;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut st = vec![];
        for c in s.chars() {
            if let Some(last) = st.last() {
                if (c.is_lowercase() && *last == (c as u8 - 32) as char)
                    || (c.is_uppercase() && *last == (c as u8 + 32) as char)
                {
                    st.pop();
                    continue;
                }
            }
            st.push(c);
        }
        st.into_iter().collect()
    }
}
