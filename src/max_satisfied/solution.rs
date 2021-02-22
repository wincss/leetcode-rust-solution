use crate::*;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let x = x as usize;
        let mut customers = customers;
        let mut total = 0;
        let mut unhappy = 0;
        for (i, v) in customers.iter_mut().enumerate() {
            total += *v;
            *v *= grumpy[i];
            unhappy += *v;
        }
        let mut happy = 0;
        let mut max_happy = 0;
        for (i, &v) in customers.iter().enumerate() {
            happy += v;
            if i >= x {
                happy -= customers[i - x];
            }
            max_happy = max_happy.max(happy);
        }
        total - unhappy + max_happy
    }
}
