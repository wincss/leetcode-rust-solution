use crate::*;

impl Solution {
    pub fn traffic_command(directions: Vec<String>) -> i32 {
        let directions: Vec<Vec<usize>> = directions
            .into_iter()
            .map(|v| {
                v.chars()
                    .map(|c| match c {
                        'E' => 0,
                        'S' => 1,
                        'W' => 2,
                        'N' => 3,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        // number the intersection point in the picture
        // top to bottom, left to right. see numbering.png
        let collision = vec![
            0b001000000000, // EE, useless
            0b110101000000, // ES
            0b000000011100, // EW
            0b000000000001, // EN
            0b001000000000, // SE
            0b100000000000, // SS, useless
            0b010010100100, // SW
            0b000100010001, // SN
            0b001110000000, // WE
            0b100000000000, // WS
            0b000000000100, // WW, useless
            0b000000101011, // WN
            0b001001010010, // NE
            0b100010001000, // NS
            0b000000000100, // NW
            0b000000000001, // NN, useless
        ];
        let range: Vec<usize> = directions.iter().map(|v| v.len()).collect();
        let mut dp = vec![
            vec![vec![vec![std::i32::MAX; range[3] + 2]; range[2] + 2]; range[1] + 2];
            range[0] + 2
        ];
        dp[range[0]][range[1]][range[2]][range[3]] = 0;
        for a in (0..=range[0]).rev() {
            for b in (0..=range[1]).rev() {
                for c in (0..=range[2]).rev() {
                    for d in (0..=range[3]).rev() {
                        let idx = [a, b, c, d];
                        let mut min = std::i32::MAX;
                        for i in 1..16 {
                            let mut ok = true;
                            let mut occupied = 0;
                            for j in 0..4 {
                                if i & (1 << j) > 0 {
                                    if idx[j] >= range[j] {
                                        continue;
                                    }
                                    let key = j * 4 + directions[j][idx[j]];
                                    if occupied & collision[key] > 0 {
                                        ok = false;
                                        break;
                                    }
                                    occupied |= collision[key];
                                }
                            }
                            if ok {
                                min = min.min(
                                    dp[a + (i & 1)][b + ((i & 2) >> 1)][c + ((i & 4) >> 2)]
                                        [d + ((i & 8) >> 3)]
                                        .saturating_add(1),
                                );
                            }
                        }
                        dp[a][b][c][d] = min.min(dp[a][b][c][d]);
                    }
                }
            }
        }
        dp[0][0][0][0]
    }
}
