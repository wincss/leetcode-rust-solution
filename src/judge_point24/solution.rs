use crate::*;

use std::ops;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Fractions(i32, i32);

impl Fractions {
    fn new(m: i32, n: i32) -> Self {
        if m == 0 {
            return Self(0, 1);
        }
        let mut a = m.abs();
        let mut b = n.abs();
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        Self(m / a * n.signum(), n / a * n.signum())
    }
}

impl ops::Add<Self> for Fractions {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1 + self.1 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::Sub<Self> for Fractions {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1 - self.1 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::Mul<Self> for Fractions {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::Div<Self> for Fractions {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        fn can_make(nums: &Vec<Fractions>, target: Fractions) -> bool {
            let n = nums.len();
            if n == 1 {
                return nums[0] == target;
            }
            for i in 0..n {
                let mut nums_copy = nums.clone();
                let this = nums_copy.remove(i);
                if can_make(&nums_copy, target + this)
                    || can_make(&nums_copy, target - this)
                    || can_make(&nums_copy, target * this)
                    || can_make(&nums_copy, target / this)
                    || can_make(&nums_copy, this / target)
                    || can_make(&nums_copy, this - target)
                {
                    // println!("0 use {:?} make {:?}", nums, target);
                    return true;
                }
            }
            if n == 4 {
                for i in 0..n {
                    for j in i + 1..n {
                        let mut nums_copy = nums.clone();
                        let n1 = nums_copy.remove(j);
                        let n2 = nums_copy.remove(i);
                        if can_make(&nums_copy, target + (n1 * n2))
                            || can_make(&nums_copy, target - (n1 * n2))
                            || can_make(&nums_copy, target + (n1 / n2))
                            || can_make(&nums_copy, target - (n1 / n2))
                        {
                            // println!("1 use {:?} make {:?}, {:?}, {:?}", nums, target, n1, n2);
                            return true;
                        }
                        if n1 + n2 != Fractions(0, 1)
                            && (can_make(&nums_copy, target * (n1 + n2))
                                || can_make(&nums_copy, target / (n1 + n2)))
                        {
                            // println!("2 use {:?} make {:?}", nums, target);
                            return true;
                        }
                        if n1 - n2 != Fractions(0, 1)
                            && (can_make(&nums_copy, target / (n1 - n2))
                                || can_make(&nums_copy, target * (n1 - n2)))
                        {
                            // println!("3 use {:?} make {:?}", nums, target);
                            return true;
                        }
                    }
                }
            }
            // println!("fail {:?} make {:?}", nums, target);
            return false;
        }
        let nums = nums
            .into_iter()
            .map(|v| Fractions(v, 1))
            .collect::<Vec<Fractions>>();
        can_make(&nums, Fractions(24, 1))
    }
}
