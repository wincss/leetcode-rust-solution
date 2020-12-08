use crate::*;

impl Solution {
    pub fn split_into_fibonacci(s: String) -> Vec<i32> {
        let s: Vec<i32> = s.chars().map(|v| v as u8 as i32 - 48).collect();
        let n = s.len();

        for (i, ov1) in s
            .iter()
            .scan(Some(0_i32), |st, v| {
                *st = (*st)
                    .and_then(|u| u.checked_mul(10))
                    .and_then(|u| u.checked_add(*v));
                st.as_ref().map(|v| *v)
            })
            .enumerate()
        {
            for (j, ov2) in s[i + 1..]
                .iter()
                .scan(Some(0_i32), |st, v| {
                    *st = (*st)
                        .and_then(|u| u.checked_mul(10))
                        .and_then(|u| u.checked_add(*v));
                    st.as_ref().map(|v| *v)
                })
                .enumerate()
            {
                let mut p = i + j + 2;
                if p == n {
                    continue;
                }
                let mut v1 = ov1;
                let mut v2 = ov2;
                // println!("v1={}, v2={}", v1, v2);

                let mut result = vec![v1, v2];

                let mut found = true;
                while found && p < n {
                    found = false;
                    let v3 = v1.checked_add(v2);
                    if v3.is_none() {
                        break;
                    }
                    let v3 = v3.unwrap();
                    for v in s[p..].iter().scan(Some(0_i32), |st, v| {
                        *st = (*st)
                            .and_then(|u| u.checked_mul(10))
                            .and_then(|u| u.checked_add(*v));
                        st.as_ref().map(|v| *v)
                    }) {
                        // println!("v={}, v3={:?}", v, v3);
                        p += 1;
                        if v == v3 {
                            v1 = v2;
                            v2 = v;
                            result.push(v);
                            found = true;
                            break;
                        } else if v > v3 {
                            break;
                        }
                    }
                    if !found {
                        break;
                    }
                }
                if found {
                    return result;
                }
                if ov2 == 0 {
                    break;
                }
            }
            if ov1 == 0 {
                break;
            }
        }
        vec![]
    }
}
