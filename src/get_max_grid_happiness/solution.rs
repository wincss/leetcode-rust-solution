use crate::*;

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        fn dp(
            k: usize,
            lastn: usize,
            ip: usize,
            ep: usize,
            m: usize,
            n: usize,
            modulus: usize,
            saved: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) -> i32 {
            if k == m * n {
                return 0;
            }
            if saved[k][lastn][ip][ep] > -1 {
                return saved[k][lastn][ip][ep];
            }
            // base: score, ip_to_reduce, ep_to_reduce
            let base = [[0, 0, 0], [120, 1, 0], [40, 0, 1]];
            // fix[x][y]: score to reduce when current cell is x and up or left cell is y
            let fix = [[0, 0, 0], [0, 60, 10], [0, 10, -40]];
            let up = lastn * 3 / modulus;
            let left = if k % n == 0 { 0 } else { lastn % 3 };
            let mut maxans = 0;
            for i in 0..=2 {
                if ip >= base[i][1] && ep >= base[i][2] {
                    maxans = std::cmp::max(
                        maxans,
                        base[i][0] as i32
                            + dp(
                                k + 1,
                                lastn * 3 % modulus + i,
                                ip - base[i][1],
                                ep - base[i][2],
                                m,
                                n,
                                modulus,
                                saved,
                            )
                            - fix[i][up]
                            - fix[i][left],
                    );
                }
            }

            saved[k][lastn][ip][ep] = maxans;
            maxans
        }
        let modulus = 3_i32.pow(n as u32) as usize;
        let mut saved =
            vec![
                vec![
                    vec![vec![-1; 1 + extroverts_count as usize]; 1 + introverts_count as usize];
                    modulus
                ];
                (m * n) as usize
            ];
        dp(
            0,
            0,
            introverts_count as usize,
            extroverts_count as usize,
            m as usize,
            n as usize,
            modulus,
            &mut saved,
        )
    }
}
