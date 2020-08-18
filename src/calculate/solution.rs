use crate::*;

use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
enum Token<'a, T> {
    Number(T),
    Operator(&'a OperatorInfo<T>),
    Parentheses(bool),
}

type F<T> = dyn Fn(&mut Vec<T>) -> ();

struct OperatorInfo<T> {
    symbol: &'static str,
    operation: Box<F<T>>,
    precedence: u8,
}

impl<T> Debug for OperatorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "\"{}\"", self.symbol)
    }
}

impl<T> PartialEq for OperatorInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.precedence == other.precedence
    }
}

impl<T> PartialOrd for OperatorInfo<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precedence.partial_cmp(&other.precedence)
    }
}

struct OperatorMap<T>(HashMap<&'static str, OperatorInfo<T>>);

impl<T> Deref for OperatorMap<T> {
    type Target = HashMap<&'static str, OperatorInfo<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OperatorMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> OperatorMap<T> {
    fn add(&mut self, info: OperatorInfo<T>) {
        self.0.insert(info.symbol, info);
    }
}

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
        fn generate_rpn(tokens: Vec<Token<i32>>) -> Vec<Token<i32>> {
            let mut result = vec![];
            let mut ops = vec![];
            for i in tokens.into_iter() {
                match i {
                    j @ Token::Number(_) => result.push(j),
                    j @ Token::Parentheses(true) => ops.push(j),
                    Token::Operator(v) => {
                        while let Some(token) = ops.last() {
                            if let Token::Operator(v0) = token {
                                if v0.precedence < v.precedence {
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
        fn calc_rpn(tokens: Vec<Token<i32>>) -> i32 {
            let mut stack = vec![];
            for i in tokens.into_iter() {
                match i {
                    Token::Number(v) => stack.push(v),
                    Token::Operator(v) => {
                        (v.operation)(&mut stack);
                    }
                    _ => unreachable!(),
                }
            }
            assert!(stack.len() == 1);
            stack.pop().unwrap()
        }
        let mut operators = OperatorMap(HashMap::new());
        operators.add(OperatorInfo {
            symbol: "+",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b + a);
            }),
            precedence: 1,
        });
        operators.add(OperatorInfo {
            symbol: "-",
            operation: Box::new(|st: &mut Vec<i32>| {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
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
        let rpn = generate_rpn(expr);
        // println!("{:?}", rpn);
        calc_rpn(rpn)
    }
}
