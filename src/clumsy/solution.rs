use crate::*;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        enum Op {
            AddMulDiv,
            MinusMulDiv,
            Add,
        }
        impl Op {
            fn next(&self) -> Self {
                match self {
                    Op::AddMulDiv => Op::Add,
                    Op::MinusMulDiv => Op::Add,
                    Op::Add => Op::MinusMulDiv,
                }
            }
            fn sig(&self) -> i32 {
                match self {
                    Op::AddMulDiv | Op::Add => 1,
                    Op::MinusMulDiv => -1,
                }
            }
        }
        let mut n = n;
        let mut result = 0;
        let mut op = Op::AddMulDiv;
        while n > 0 {
            match op {
                Op::AddMulDiv | Op::MinusMulDiv => {
                    result += op.sig() * if n >= 3 { n * (n - 1) / (n - 2) } else { n };
                    n = 0.max(n - 3);
                }
                Op::Add => {
                    result += n;
                    n -= 1;
                }
            }
            op = op.next();
        }
        result
    }
}
