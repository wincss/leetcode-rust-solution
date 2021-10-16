use crate::*;
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        fn dfs(
            num: &str,
            target: i64,
            v1: i64,
            v2: i64,
            current: &mut String,
            result: &mut Vec<String>,
        ) {
            if num.len() == 0 {
                if v1 + v2 == target {
                    result.push(current.clone());
                }
                return;
            }
            let l = current.len();
            if l > 0 {
                current.push(' ');
            }
            let mut v = 0;
            for (offset, c) in num.char_indices() {
                if offset > 0 && v == 0 {
                    break;
                }
                current.push(c);
                v = v * 10 + c as u8 as i64 - 48;
                if l == 0 {
                    dfs(&num[offset + 1..], target, v1 + v2, v, current, result);
                    continue;
                }
                current.replace_range(l..l + 1, &"+");
                dfs(&num[offset + 1..], target, v1 + v2, v, current, result);
                current.replace_range(l..l + 1, &"-");
                dfs(&num[offset + 1..], target, v1 + v2, -v, current, result);
                current.replace_range(l..l + 1, &"*");
                dfs(&num[offset + 1..], target, v1, v2 * v, current, result);
            }
            current.truncate(l);
        }
        let mut result = vec![];
        let mut current = String::new();
        dfs(&num, target as i64, 0, 0, &mut current, &mut result);
        result
    }
}
