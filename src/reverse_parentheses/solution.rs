use crate::*;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut st = vec![];
        let mut rev = vec![];
        for c in s.chars() {
            match c {
                ')' => {
                    while let Some(v) = st.pop() {
                        match v {
                            '(' => {
                                st.append(&mut rev);
                                break;
                            }
                            _ => rev.push(v),
                        }
                    }
                }
                _ => st.push(c),
            }
            // println!("{:?} {:?}", st, rev);
        }
        st.into_iter().collect()
    }
}
