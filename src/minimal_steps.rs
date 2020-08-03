use crate::Solution;
use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        let n = maze.len() as i32;
        let m = maze[0].len() as i32;
        let maze: Vec<&[u8]> = maze.iter().map(|x| x.as_bytes()).collect();
        let mut ml = vec![];
        let mut od = HashMap::new();
        let mut s = 0;
        let mut t = 0;
        for (i, &line) in maze.iter().enumerate() {
            for (j, &c) in line.iter().enumerate() {
                match c {
                    b'S' | b'M' => {
                        ml.push((i, j));
                        if c == b'S' {
                            s = ml.len() - 1;
                        }
                    }
                    b'O' | b'T' => {
                        od.insert((i, j), od.len());
                        if c == b'T' {
                            t = od.len() - 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        // println!("s = {}, t = {}", s, t);
        // println!("ml = {:?}", ml);
        // println!("od = {:?}", od);

        let mut m2o = vec![];
        for &s in ml.iter() {
            // println!("start from {:?}", s);
            let mut d = vec![std::i32::MAX; od.len()];
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((s, 0));
            visited.insert(s);
            while let Some((p, distance)) = queue.pop_front() {
                // println!("go {:?} in {}", p, distance);
                if let Some(&oid) = od.get(&p) {
                    d[oid] = distance;
                }
                let (x, y) = p;
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                    let (x, y) = (x as i32 + dx, y as i32 + dy);
                    // print!("{},{}", x, y);
                    if x < 0 || x >= n || y < 0 || y >= m {
                        // println!("continue 1");
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if maze[x][y] == b'#' || visited.contains(&(x, y)) {
                        // println!("continue 2");
                        continue;
                    }
                    // println!("expand");
                    queue.push_back(((x, y), distance + 1));
                    visited.insert((x, y));
                }
            }
            m2o.push(d);
        }
        let mut m2m = vec![];
        for i in 0..ml.len() {
            let mut i2m = vec![];
            for j in 0..ml.len() {
                if i == j {
                    i2m.push(0);
                } else {
                    i2m.push(
                        (0..od.len())
                            .map(|k| {
                                if k == t {
                                    std::i32::MAX
                                } else {
                                    m2o[i][k].saturating_add(m2o[j][k])
                                }
                            })
                            .min()
                            .unwrap(),
                    );
                }
            }
            m2m.push(i2m);
        }
        // println!("m2o = {:?}", m2o);
        // println!("m2m = {:?}", m2m);

        fn dp(
            now: usize,
            t: usize,
            m_num: usize,
            left: i32,
            m2o: &Vec<Vec<i32>>,
            m2m: &Vec<Vec<i32>>,
            mut save: &mut HashMap<(usize, i32), i32>,
        ) -> i32 {
            if left == 0 {
                return m2o[now][t];
            }
            let key = (now, left);
            if let Some(&v) = save.get(&key) {
                return v;
            }
            let mut ans = std::i32::MAX;
            for i in 0..m_num {
                if left & (1 << i) != 0 {
                    ans = cmp::min(
                        ans,
                        m2m[now][i].saturating_add(dp(
                            i,
                            t,
                            m_num,
                            left ^ (1 << i),
                            &m2o,
                            &m2m,
                            &mut save,
                        )),
                    )
                }
            }
            save.insert(key, ans);
            return ans;
        }
        let mut save = HashMap::new();
        match dp(
            s,
            t,
            ml.len(),
            (1 << ml.len()) - 1 - (1 << s),
            &m2o,
            &m2m,
            &mut save,
        ) {
            std::i32::MAX => -1,
            ans => ans,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::minimal_steps(to_string_vec(&["S#O", "M..", "M.T"])),
            16
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::minimal_steps(to_string_vec(&["S#O", "M.#", "M.T"])),
            -1
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::minimal_steps(to_string_vec(&["S#O", "M.T", "M.."])),
            17
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::minimal_steps(to_string_vec(&["MMMMM", "MS#MM", "MM#TO"])),
            95
        );
    }
}
