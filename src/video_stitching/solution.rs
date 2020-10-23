use crate::*;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let t = t as usize;
        let mut ending = vec![0; t];
        for span in clips.into_iter() {
            for i in span[0]..std::cmp::min(span[1], t as i32) {
                ending[i as usize] = std::cmp::max(ending[i as usize], span[1] as usize);
            }
        }
        let mut result = 0;
        let mut cover = 0;
        while cover < t {
            if ending[cover] <= cover {
                return -1;
            }
            cover = ending[cover];
            result += 1;
        }
        result
    }
}
