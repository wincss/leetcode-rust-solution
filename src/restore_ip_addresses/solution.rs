use crate::*;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn helper(s: &[u8], n: i32) -> Vec<Vec<u8>> {
            match (n, s.len()) {
                (0, 0) => vec![vec![]],
                (0, _) => vec![],
                (_, 0) => vec![],
                _ => {
                    let mut ans = vec![];
                    let mut num = 0_u8;
                    for i in 0..s.len() {
                        if num == 0 && i > 0 {
                            break;
                        }
                        if let Some(v) = num.checked_mul(10).and_then(|v| v.checked_add(s[i] - 48))
                        {
                            num = v;
                        } else {
                            break;
                        }
                        ans.extend(helper(&s[i + 1..], n - 1).into_iter().map(|mut v| {
                            let mut r = vec![num];
                            r.append(&mut v);
                            r
                        }));
                    }
                    ans
                }
            }
        }
        helper(s.as_bytes(), 4)
            .into_iter()
            .map(|i| {
                i.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
            })
            .collect::<Vec<String>>()
    }
}
