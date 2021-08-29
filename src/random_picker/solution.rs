use rand::Rng;

#[allow(dead_code)]
pub struct RandomPicker {
    weight: Vec<i32>,
    sum: i32,
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomPicker {
    #[allow(dead_code)]
    pub fn new(w: Vec<i32>) -> Self {
        let sum: i32 = w.iter().map(|x| *x).sum();
        Self { weight: w, sum }
    }
    #[allow(dead_code)]
    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut r = rng.gen_range(0, self.sum);
        for (i, &w) in self.weight.iter().enumerate() {
            if r < w {
                return i as i32;
            }
            r -= w;
        }
        unreachable!();
    }
}
