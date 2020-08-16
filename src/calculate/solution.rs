use crate::*;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Number(i32),
    Operator(String),
    Parentheses(bool),
}

type F = dyn Fn(&mut Vec<i32>) -> ();

struct OperatorInfo {
    symbol: &'static str,
    operation: Box<F>,
    precedence: u8,
}

type OperatorMap = HashMap<&'static str, OperatorInfo>;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        fn parse_expression(s: &String, operators: &OperatorMap) -> Vec<Token> {
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
                            result.push(Token::Operator((v as char).to_string()));
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
        fn generate_rpn(tokens: Vec<Token>, operators: &OperatorMap) -> Vec<Token> {
            let mut result = vec![];
            let mut ops = vec![];
            for i in tokens.into_iter() {
                match i {
                    j @ Token::Number(_) => result.push(j),
                    j @ Token::Parentheses(true) => ops.push(j),
                    Token::Operator(v) => {
                        while let Some(token) = ops.last() {
                            if let Token::Operator(v0) = token {
                                if operators[&v0[..]].precedence < operators[&v[..]].precedence {
                                    break;
                                }
                                result.push(ops.pop().unwrap())
                            } else {
                                break;
                            }
                        }
                        ops.push(Token::Operator(v));
                    }
                    Token::Parentheses(false) => {
                        while let Some(token) = ops.pop() {
                            if let Token::Parentheses(_) = token {
                                break;
                            }
                            result.push(token);
                        }
                    }
                }
            }
            while let Some(token) = ops.pop() {
                result.push(token);
            }
            result
        }
        fn calc_rpn(tokens: Vec<Token>, operators: &OperatorMap) -> i32 {
            let mut stack = vec![];
            for i in tokens.into_iter() {
                match i {
                    Token::Number(v) => stack.push(v),
                    Token::Operator(v) => {
                        (operators[&v[..]].operation)(&mut stack);
                    }
                    _ => unreachable!(),
                }
            }
            assert!(stack.len() == 1);
            stack.pop().unwrap()
        }
        let mut operators: OperatorMap = HashMap::new();
        operators.insert(
            "+",
            OperatorInfo {
                symbol: "+",
                operation: Box::new(|st: &mut Vec<i32>| {
                    let a = st.pop().unwrap();
                    let b = st.pop().unwrap();
                    st.push(b + a);
                }),
                precedence: 1,
            },
        );
        operators.insert(
            "-",
            OperatorInfo {
                symbol: "-",
                operation: Box::new(|st: &mut Vec<i32>| {
                    let a = st.pop().unwrap();
                    let b = st.pop().unwrap();
                    st.push(b - a);
                }),
                precedence: 1,
            },
        );
        operators.insert(
            "*",
            OperatorInfo {
                symbol: "*",
                operation: Box::new(|st: &mut Vec<i32>| {
                    let a = st.pop().unwrap();
                    let b = st.pop().unwrap();
                    st.push(b * a);
                }),
                precedence: 2,
            },
        );
        operators.insert(
            "/",
            OperatorInfo {
                symbol: "/",
                operation: Box::new(|st: &mut Vec<i32>| {
                    let a = st.pop().unwrap();
                    let b = st.pop().unwrap();
                    st.push(b / a);
                }),
                precedence: 2,
            },
        );
        let expr = parse_expression(&s, &operators);
        // println!("{:?}", expr);
        let rpn = generate_rpn(expr, &operators);
        // println!("{:?}", rpn);
        calc_rpn(rpn, &operators)
    }
}
