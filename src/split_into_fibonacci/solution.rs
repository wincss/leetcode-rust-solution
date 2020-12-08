use crate::*;

impl Solution {
    pub fn split_into_fibonacci(s: String) -> Vec<i32> {
        fn digit_accumulator<'a>(s: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
            s.iter().scan(Some(0_i32), |st, v| {
                *st = (*st)
                    .and_then(|u| u.checked_mul(10))
                    .and_then(|u| u.checked_add(*v));
                st.as_ref().map(|v| *v)
            })
        }

        let s: Vec<i32> = s.chars().map(|v| v as u8 as i32 - 48).collect();
        let n = s.len();

        for (i, ov1) in digit_accumulator(&s[..]).enumerate() {
            for (j, ov2) in digit_accumulator(&s[i + 1..]).enumerate() {
                let mut p = i + j + 2;
                let mut v1 = ov1;
                let mut v2 = ov2;
                let mut result = vec![v1, v2];
                while let Some(v3) = v1.checked_add(v2) {
                    let mut found = false;
                    for v in digit_accumulator(&s[p..]) {
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
                    if p == n {
                        return result;
                    }
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
