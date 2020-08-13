use crate::*;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = Vec::new();
        for i in s.chars() {
            match i {
                '(' => {
                    st.push(')');
                }
                '[' => {
                    st.push(']');
                }
                '{' => {
                    st.push('}');
                }
                ')' | ']' | '}' => match st.pop() {
                    None => {
                        return false;
                    }
                    Some(v) if v != i => {
                        return false;
                    }
                    Some(_) => {}
                },
                _ => unreachable!(),
            }
        }
        st.is_empty()
    }
}
