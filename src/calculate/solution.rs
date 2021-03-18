use crate::*;

use common::algorithms::rpn::*;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn parse_expression<'a>(
            s: &String,
            operators: &'a OperatorMap<i32>,
        ) -> Vec<Token<'a, i32>> {
            let mut result = vec![];
            let mut last_number = None;
            for &i in s.as_bytes() {
                if i.is_ascii_digit() {
                    last_number.replace(last_number.unwrap_or(0) * 10 + i as i32 - 48);
                } else {
                    if let Some(v) = last_number.take() {
                        result.push(Token::Number(v))
                    }
                    match i {
                        b'(' | b'[' | b'{' => {
                            result.push(Token::Parentheses(true));
                        }
                        b')' | b']' | b'}' => {
                            result.push(Token::Parentheses(false));
                        }
                        b' ' => {}
                        v if operators.contains_key(&(v as char).to_string()[..]) => {
                            result.push(Token::Operator(
                                operators.get(&(v as char).to_string()[..]).unwrap(),
                            ));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            if let Some(v) = last_number.take() {
                result.push(Token::Number(v));
            }
            result
        }

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
        let expr = parse_expression(&s, &operators);
        // println!("{:?}", expr);
        let rpn = operators.generate_rpn(expr);
        // println!("{:?}", rpn);
        operators.calc_rpn(rpn)
    }
}
