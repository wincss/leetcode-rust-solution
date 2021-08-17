use crate::*;

impl Solution {
    pub fn check_record_552(n: i32) -> i32 {
        const MOD: i64 = 1000000007;
        // state:
        // .0 .1 .2 => absent 0, late 0, 1, 2
        // .3 .4 .5 => absent 1, late 0, 1, 2
        let mut state = (1, 0, 0, 0, 0, 0);
        for _ in 0..n {
            state = (
                (state.0 + state.1 + state.2) % MOD,
                state.0,
                state.1,
                (state.0 + state.1 + state.2 + state.3 + state.4 + state.5) % MOD,
                state.3,
                state.4,
            );
        }
        ((state.0 + state.1 + state.2 + state.3 + state.4 + state.5) % MOD) as i32
    }
    pub fn check_record_551(s: String) -> bool {
        let mut absent = 0;
        let mut late = 0;
        let mut continous_late = false;
        for c in s.chars() {
            if c == 'A' {
                absent += 1;
            }
            if c == 'L' {
                late += 1;
                if late == 3 {
                    continous_late = true;
                }
            } else {
                late = 0;
            }
        }
        absent < 2 && !continous_late
    }
}
