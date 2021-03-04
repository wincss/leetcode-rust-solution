use crate::*;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        let mut saved = vec![0; n];
        fn max_envelopes_internal(
            k: usize,
            n: usize,
            envelopes: &Vec<Vec<i32>>,
            saved: &mut Vec<i32>,
        ) -> i32 {
            if saved[k] > 0 {
                return saved[k];
            }
            let mut result = 1;
            for i in 0..n {
                if envelopes[k][0] > envelopes[i][0] && envelopes[k][1] > envelopes[i][1] {
                    result = result.max(max_envelopes_internal(i, n, envelopes, saved) + 1);
                }
            }
            saved[k] = result;
            result
        }
        (0..n)
            .map(|k| max_envelopes_internal(k, n, &envelopes, &mut saved))
            .max()
            .unwrap()
    }
}
