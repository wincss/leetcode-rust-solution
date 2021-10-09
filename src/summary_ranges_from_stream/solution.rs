pub struct SummaryRanges {
    data: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    #[allow(dead_code)]
    pub fn add_num(&mut self, val: i32) {
        let idx = self
            .data
            .binary_search(&vec![val, val])
            .unwrap_or_else(|x| x);
        println!("{}, {:?}", idx, self.data);
        if idx > 0 {
            if val <= self.data[idx - 1][1] {
                return;
            } else if val == self.data[idx - 1][1] + 1 {
                if idx < self.data.len() && self.data[idx][0] == val + 1 {
                    self.data[idx - 1][1] = self.data[idx][1];
                    self.data.remove(idx);
                } else {
                    self.data[idx - 1][1] += 1;
                }
                return;
            }
        }
        if idx < self.data.len() {
            if val == self.data[idx][0] {
                return;
            } else if val == self.data[idx][0] - 1 {
                self.data[idx][0] -= 1;
                return;
            }
        }
        self.data.insert(idx, vec![val, val]);
    }

    #[allow(dead_code)]
    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.data.clone()
    }
}
