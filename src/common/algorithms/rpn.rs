use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub enum Token<'a, T> {
    Number(T),
    Operator(&'a OperatorInfo<T>),
    Parentheses(bool),
}

type F<T> = dyn Fn(&mut Vec<T>) -> ();

pub struct OperatorInfo<T> {
    pub symbol: &'static str,
    pub operation: Box<F<T>>,
    pub precedence: u8,
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

pub struct OperatorMap<T>(HashMap<&'static str, OperatorInfo<T>>);

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

impl<'a, T> OperatorMap<T> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, info: OperatorInfo<T>) {
        self.0.insert(info.symbol, info);
    }

    pub fn generate_rpn(&self, tokens: Vec<Token<'a, T>>) -> Vec<Token<'a, T>> {
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

    pub fn calc_rpn(&self, tokens: Vec<Token<T>>) -> T {
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
}
