use crate::*;

use common::algorithms::rpn::*;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operators = OperatorMap::new();
        operators.add(OperatorInfo {
            symbol: "+",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                // add can be a unary operator
                let b = st.pop().unwrap_or(0);
                st.push(b + a);
            }),
            precedence: 1,
        });
        operators.add(OperatorInfo {
            symbol: "-",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                // minus can be a unary operator
                let b = st.pop().unwrap_or(0);
                st.push(b - a);
            }),
            precedence: 1,
        });
        operators.add(OperatorInfo {
            symbol: "*",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b * a);
            }),
            precedence: 2,
        });
        operators.add(OperatorInfo {
            symbol: "/",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b / a);
            }),
            precedence: 2,
        });
        let mut rpn = vec![];
        for i in tokens.into_iter() {
            match &i[..] {
                "+" | "-" | "*" | "/" => rpn.push(Token::Operator(operators.get(&i[..]).unwrap())),
                _ => rpn.push(Token::Number(i.parse::<i32>().unwrap())),
            }
        }
        operators.calc_rpn(rpn)
    }
}
