use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let ls = s.len();
        let lp = p.len();

        let mut failed = HashSet::new();
        let mut stack = vec![];
        stack.push((0, 0));
        while let Some(v) = stack.pop() {
            if failed.contains(&v) {
                continue;
            }
            let (mut ps, mut pp) = v.clone();
            while pp < lp {
                if pp + 1 < lp && p[pp + 1] == '*' {
                    while ps < ls && (s[ps] == p[pp] || p[pp] == '.') {
                        stack.push((ps, pp + 2));
                        ps += 1;
                    }
                    pp += 2;
                } else if ps < ls {
                    if s[ps] != p[pp] && p[pp] != '.' {
                        break;
                    }
                    ps += 1;
                    pp += 1;
                } else {
                    break;
                }
            }
            if ps == ls && pp == lp {
                return true;
            }
            failed.insert(v);
        }
        false
    }
}
